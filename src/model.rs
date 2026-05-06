use core::range;

use ndarray::Array2;

struct Model {
    pub theta_0: f64,
    pub theta_1: f64,
}

impl Model {
    fn fit(&mut self, data: &Array2<f64>, alpha: f64, epochs: i64) {
        self.theta_0 = 0.0;
        self.theta_0 = 0.0;

        // column은 뷰어 -> 레퍼런스,
        // 소유권 필요 시 .to_owned 사용. 이 경우 힙에 동적할당
        let x = data.column(1); // km     
        let y = data.column(0); // price
        // as f64 사용 이윺는 아래 residual.sum()이 f64이기 때문
        // 다른 숫자 자료형 간 나눗셈은 허용하지 않음
        let m = data.nrows() as f64; // 데이터의 수

        for epoch in 0..epochs {
            // 스칼라 벡터 간 바인딩 후 연산을 자동으로 지원하는 개멋진 러스트!
            // 힙 할당 타입의 연산자의 경우 소유권이 이동될 수 있기 때문에 레퍼런스 활용
            let y_hat = self.theta_0 + self.theta_1 * &x;
            let residual = &y - &y_hat;

            let d_theta_0 = residual.sum() / m;
            let d_theta_1 = (&residual * &x).sum() / m;

            self.theta_0 -= alpha * d_theta_0;
            self.theta_1 -= alpha * d_theta_1;

            let rmse = &residual.mapv(|x| x.powi(2)).mean().unwrap().sqrt();
            println!("epoch: {epoch}: rmse: {rmse}");
        }
    }

    fn load_params(&mut self, params: &Array2<f64>) {
        self.theta_0 = *params.get((0, 1)).unwrap();
        self.theta_1 = *params.get((0, 1)).unwrap();
    }
}
