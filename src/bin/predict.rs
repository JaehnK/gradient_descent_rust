// use clap::Parser;
use ft_linear_regression::*;
use std::io;

// #[derive(Parser)]
// struct Args {
//     /// mileage
//     #[clap(short, long)]
//     mileage: f64,
// }

// impl Args {
//     fn validate(&self) -> Result<(), String> {
//         if self.mileage <= 0.0 {
//             return Err(format!(
//                 "mileage must be positive number, got {}",
//                 self.mileage
//             ));
//         }
//         Ok(())
//     }
// }

fn main() -> Result<(), String> {
    // let args = Args::parse();
    // args.validate().unwrap();
    // println!("Input mileage: {}", args.mileage);

    let mut input = String::new();

    println!("Enter mileage:");
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    let mileage: f64 = input
        .trim()
        .parse()
        .map_err(|e: std::num::ParseFloatError| e.to_string())?;

    println!("Input mileage: {}", mileage);
    let params = read_yaml()?;

    let model = model::Model {
        theta_0: params.theta_0,
        theta_1: params.theta_1,
    };
    println!(
        "Model parameters: theta_0: {:.3}, theta_1: {:.3}",
        model.theta_0, model.theta_1
    );
    let prediction = model.predict(mileage, Some(&params));
    println!("Predicted price: {:.3}", prediction);
    Ok(())
}
