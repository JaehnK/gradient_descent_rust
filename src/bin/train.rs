use clap::Parser;
use std::path::PathBuf;


#[derive(Parser)]
struct Args {
    /// 입력 데이터 파일 경로(csv)
    #[clap(short, long)]
    input: PathBuf,

    /// 학습률(alpha)
    #[clap(short, long, default_value = "0.01")]
    alpha: f64,
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
        Ok(())
    }
}

fn main() {
    let args = Args::parse();
    args.validate_input_path().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    println!("Input: {}", args.input.display());
    println!("Alpha: {}", args.alpha);
}