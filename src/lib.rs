use chrono::Local;
use csv::Reader;
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use statrs::statistics::Statistics;
use std::io::Write;
use std::path::Path;

pub mod model;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelParams {
    pub theta_0: f64,
    pub theta_1: f64,
}

pub struct NormalisedData {
    pub data: Array2<f64>,
    pub x_mean: f64,
    pub x_std: f64,
}

pub fn read_csv(path: &Path) -> Result<Array2<f64>, String> {
    let mut reader: Reader<std::fs::File> = Reader::from_path(path).map_err(|e| e.to_string())?;

    // 헤더 검사 로직 km, price가 아님 에러 리턴
    let headers: &csv::StringRecord = reader.headers().map_err(|e: csv::Error| e.to_string())?;
    if headers.get(0) != Some("km") || headers.get(1) != Some("price") {
        return Err("CSV header must be km and price".to_string());
    };

    let mut rows: Vec<f64> = Vec::new();
    let mut nrows: usize = 0;

    for i in reader.records() {
        let record: csv::StringRecord = i.map_err(|e| e.to_string())?;
        if record.len() != 2 {
            return Err(format!("Expected 2 Columns, got {}", record.len()));
        }

        for value in record.iter() {
            let v: f64 = value
                .trim()
                .parse()
                .map_err(|e: std::num::ParseFloatError| e.to_string())?;
            rows.push(v);
        }
        nrows += 1;
    }

    Array2::from_shape_vec((nrows, 2), rows).map_err(|e| e.to_string())
}

pub fn noramlise_data(data: &Array2<f64>) -> Result<NormalisedData, String> {
    // 이 함수는 z-score기반의 정규화를 활용함
    // 학습률(alpha)값에 더욱 강건하며, 평균이 0, 분산이 1로 수렴해서 gradient descent가 안정적으로 학습

    let x_n = data.nrows() as f64;
    if x_n < 2.0 {
        return Err("need more values to normalise".to_string());
    }
    let x_mean = data.column(1).mean();
    let sum_squared_diff = (&data.column(1) - x_mean).mapv(|x| x.powi(2)).sum();
    let x_std = (sum_squared_diff / (x_n - 1.0)).sqrt();
    if x_std == 0.0 {
        return Err("std is zero".to_string());
    }

    let norm_x = (&data.column(1) - x_mean) / x_std;

    // data의 소유권이 없음으로 클론을 통해 다른 배열 생성
    let mut result = data.clone();
    result.column_mut(1).assign(&norm_x);

    Ok(NormalisedData {
        data: result,
        x_mean,
        x_std,
    })
}

pub fn save_model_params(model: &model::Model) -> Result<(), String> {
    let params = ModelParams {
        theta_0: model.theta_0,
        theta_1: model.theta_1,
    };

    let yaml_string = serde_yaml::to_string(&params).map_err(|e| e.to_string())?;

    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("params_{}.yaml", timestamp);
    let mut file = std::fs::File::create(&filename).map_err(|e| e.to_string())?;
    file.write_all(yaml_string.as_bytes())
        .map_err(|e| e.to_string())?;

    println!("Model parameters saved to {}", filename);
    Ok(())
}
