use csv::Reader;
use ndarray::Array2;
use std::path::Path;

pub mod model;

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
