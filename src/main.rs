use clap::Clap;
use std::str::FromStr;

pub mod nors;
use crate::nors::{CounterType, Nors, OutputType};

#[derive(Clap)]
#[clap(version = "0.1", author = "Booink <booink.work@gmail.com>")]
struct Opts {
    input: String,
    #[clap(short, long, default_value = "fill_buffer")]
    counter_type: String,
    #[clap(short, long, default_value = "plaintext")]
    output_type: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let nors = Nors::new(&opts.input);
    let counter_type = CounterType::from_str(&opts.counter_type);
    if counter_type.is_err() {
        println!("counter type error.");
        return;
    }
    let counter_type = counter_type.unwrap();

    let result = nors.count_by_type(counter_type);
    if result.is_err() {
        println!("File open error. {}", opts.input);
        return;
    }
    let result = result.unwrap();

    let output_type = OutputType::from_str(&opts.output_type);
    if output_type.is_err() {
        println!("Output type error.");
        return;
    }
    let output_type = output_type.unwrap();
    result.print(output_type);
}
