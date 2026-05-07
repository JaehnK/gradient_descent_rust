use clap::Parser;

#[derive(Parser)]
struct Args {
    /// x (mileage)
    #[clap(short, long)]
    x: f64,
}

impl Args {
    fn validate(&self) -> Result<(), String> {
        if self.x <= 0.0 {
            return Err(format!("milege must be positive number, got {}", self.x));
        }
        Ok(())
    }
}

fn main() {
    let args = Args::parse();
    args.validate().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    println!("Input X value: {}", args.x);
}
