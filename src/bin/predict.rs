use clap::Parser;

#[derive(Parser)]
struct Args {
    /// x (mileage)
    #[clap(short, long,)]
    x: f64
}

impl Args {
    fn validate(&self) -> Result<(), String> {
        if self.x <= 0.0 {
            return Err(format!("milege must be positive number, got {}", self.x));
        }
        Ok(());
    }
}

fn main() {
    let args = Args::parse();
    println!("Input X value: {}", args.x);
}