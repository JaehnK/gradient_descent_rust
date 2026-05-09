use crate::ModelParams;
use ndarray::Array2;
use plotters::prelude::*;
use std::error::Error;

pub struct Model {
    pub theta_0: f64,
    pub theta_1: f64,
    pub is_converged: bool,
}

impl Model {
    pub fn new(theta_0: f64, theta_1: f64) -> Self {
        Self {
            theta_0,
            theta_1,
            is_converged: false,
        }
    }

    pub fn fit(&mut self, data: &Array2<f64>, alpha: f64, epochs: u64) {
        // column은 뷰어 -> 레퍼런스,
        // 소유권 필요 시 .to_owned 사용. 이 경우 힙에 동적할당
        let x = data.column(0); // km     
        let y = data.column(1); // price
        // as f64 사용 이윺는 아래 residual.sum()이 f64이기 때문
        // 다른 숫자 자료형 간 나눗셈은 허용하지 않음
        let m = data.nrows() as f64; // 데이터의 수

        let mut rmse_values: Vec<f64> = Vec::new();

        for epoch in 0..epochs {
            // 스칼라 벡터 간 바인딩 후 연산을 자동으로 지원하는 개멋진 러스트!
            // 힙 할당 타입의 연산자의 경우 소유권이 이동될 수 있기 때문에 레퍼런스 활용
            let y_hat = self.theta_0 + self.theta_1 * &x;
            let residual = &y - &y_hat;

            let d_theta_0 = residual.sum() / m;
            let d_theta_1 = (&residual * &x).sum() / m;

            self.theta_0 += alpha * d_theta_0;
            self.theta_1 += alpha * d_theta_1;

            let rmse = &residual.mapv(|x| x.powi(2)).mean().unwrap().sqrt();
            println!("epoch: {epoch}: rmse: {rmse:.4}");
            rmse_values.push(*rmse);
        }

        // Plot RMSE values
        if rmse_values.len() > 1
            && rmse_values.iter().all(|&r| r.is_finite())
            && self.theta_0.is_finite()
            && self.theta_1.is_finite()
        {
            let _ = self.plot_rmse(&rmse_values);
            self.is_converged = true;
        }
    }

    fn plot_rmse(&self, rmse_values: &[f64]) -> Result<(), Box<dyn Error>> {
        let root = BitMapBackend::new("rmse.png", (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        let max_rmse = rmse_values.iter().copied().fold(0.0_f64, f64::max);

        let mut chart = ChartBuilder::on(&root)
            .caption("RMSE per Epoch", ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(30)
            .y_label_area_size(50)
            .build_cartesian_2d(0..rmse_values.len(), 0f64..max_rmse)?;

        chart.configure_mesh().draw()?;

        chart.draw_series(LineSeries::new(
            rmse_values.iter().enumerate().map(|(i, &r)| (i, r)),
            &RED,
        ))?;

        root.present()?;
        Ok(())
    }

    pub fn plot_scatterline(
        &self,
        data: &Array2<f64>,
        params: Option<&ModelParams>,
    ) -> Result<(), Box<dyn Error>> {
        let root = BitMapBackend::new("scatter_line.png", (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;

        let x = data.column(0);
        let y = data.column(1);

        let x_min = x.iter().copied().fold(f64::INFINITY, f64::min);
        let x_max = x.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let y_min = y.iter().copied().fold(f64::INFINITY, f64::min);
        let y_max = y.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        let mut chart = ChartBuilder::on(&root)
            .caption("Training Data and Trend Line", ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

        chart.configure_mesh().draw()?;

        chart.draw_series(
            x.iter()
                .zip(y.iter())
                .map(|(&x_value, &y_value)| Circle::new((x_value, y_value), 3, BLUE.filled())),
        )?;

        chart.draw_series(LineSeries::new(
            [
                (x_min, self.predict(x_min, params)),
                (x_max, self.predict(x_max, params)),
            ],
            &RED,
        ))?;

        root.present()?;
        Ok(())
    }

    pub fn predict(&self, x: f64, params: Option<&ModelParams>) -> f64 {
        let norm_x = match params {
            Some(params) => match (params.x_mean, params.x_std) {
                (Some(mean), Some(std)) => (x - mean) / std,
                _ => x,
            },
            None => x,
        };

        self.theta_0 + self.theta_1 * norm_x
    }
}
