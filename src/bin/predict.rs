use clap::Parser;

use ft_linear_regression::read_yaml;

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

fn main() -> Result<(), String> {
    let args = Args::parse();
    args.validate().unwrap();
    println!("Input X value: {}", args.x);

    let params = read_yaml()?;

    let mut model = ft_linear_regression::Model::new(params.theta_0, params.theta_1);
    let prediction = model.fit(args.x, &params);
    println!("Prediction: {}", prediction);
    Ok(())
}
