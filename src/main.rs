use clap::Parser;
use std::str::FromStr;

pub mod nors;
use crate::nors::{CounterType, Nors, OutputType};

#[derive(Parser)]
#[clap(version = "0.1", author = "Booink <booink.work@gmail.com>")]
struct Opts {
    input: String,
    #[clap(short, long, default_value = "plaintext")]
    output_type: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let nors = Nors::new(&opts.input);

    let result = nors.count_by_type(CounterType::FillBuffer);
    if let Err(e) = result {
        eprintln!("{}", e);
        return;
    }

    let output_type = OutputType::from_str(&opts.output_type);
    if let Err(e) = output_type {
        eprintln!("{}", e);
        return;
    }
    result.unwrap().print(output_type.unwrap());
}
