use clap::Parser;

use ft_linear_regression::*;

#[derive(Parser)]
struct Args {
    /// mileage
    #[clap(short, long)]
    mileage: f64,
}

impl Args {
    fn validate(&self) -> Result<(), String> {
        if self.mileage <= 0.0 {
            return Err(format!(
                "mileage must be positive number, got {}",
                self.mileage
            ));
        }
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    args.validate().unwrap();
    println!("Input mileage: {}", args.mileage);

    let params = read_yaml()?;

    let model = model::Model {
        theta_0: params.theta_0,
        theta_1: params.theta_1,
    };
    println!(
        "Model parameters: theta_0: {:.3}, theta_1: {:.3}",
        model.theta_0, model.theta_1
    );
    let prediction = model.predict(args.mileage, Some(&params));
    println!("Predicted price: {:.3}", prediction);
    Ok(())
}
