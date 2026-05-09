use clap::Parser;
use ndarray::Array2;
use std::path::PathBuf;

use ft_linear_regression::*;

#[derive(Parser)]
struct Args {
    /// 입력 데이터 파일 경로(csv)
    #[clap(short, long)]
    input: PathBuf,

    /// 학습률(alpha)
    #[clap(short, long, default_value = "0.01")]
    alpha: f64,

    /// 반복 횟수(epoch)
    #[clap(short, long)]
    epoch: u64,

    /// 변수의 정규화 여부
    #[clap(short, long)]
    normalize: bool,
}

impl Args {
    // Result<T, E>: 성공 실패를 나타내는 enum
    fn validate_input_path(&self) -> Result<(), String> {
        if !self.input.is_file() {
            return Err(format!("File Not Found: {}", self.input.display()));
        }
        // FFI — Foreign Function Interface, 외부 언어와의 인터페이스.
        // OS API가 C로 작성되어 있어서, OS와 직접 소통하는 타입들을 ffi 모듈에 존재
        if self.input.extension() != Some(std::ffi::OsStr::new("csv")) {
            return Err("Unexpected file type(not csv)".to_string());
        }
        // Ok안에 unit 타입 ()를 넣는 것입니다.
        Ok(())
    }

    fn validate_alpha(&self) -> Result<(), String> {
        if self.alpha <= 0.0 || self.alpha >= 1.0 {
            return Err(format!("alpha must be in (0, 1), got {}", self.alpha));
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), String> {
        // ? 연산자 — match의 축약
        self.validate_input_path()?;
        self.validate_alpha()?;
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let args: Args = Args::parse();
    args.validate().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    println!("Input: {}", args.input.display());
    println!("Alpha: {}", args.alpha);

    let data: Array2<f64> = read_csv(&args.input)?;

    let mut model = model::Model {
        theta_0: 0.0,
        theta_1: 0.0,
        is_converged: false,
    };

    if args.normalize == true {
        println!("Normalise has set True");
        let normalised_data = noramlise_data(&data).unwrap();
        model.fit(&normalised_data.data, args.alpha, args.epoch);
        if model.is_converged {
            let params = ModelParams {
                theta_0: model.theta_0,
                theta_1: model.theta_1,
                x_mean: Some(normalised_data.x_mean),
                x_std: Some(normalised_data.x_std),
            };
            let _ = model.plot_scatterline(&data, Some(&params));
        }
        save_model_params(
            &model,
            Some(normalised_data.x_mean),
            Some(normalised_data.x_std),
        )
    } else {
        model.fit(&data, args.alpha, args.epoch);
        if model.is_converged {
            let _ = model.plot_scatterline(&data, None);
        }
        save_model_params(&model, None, None)
    }
}
