use clap::Parser;

#[derive(Parser)]
struct Args {
    /// x (mileage)
    #[clap(short, long,)]
    x: f64
}

fn main() {
    let args = Args::parse();
    println!("Input X value: {}", args.x);
}