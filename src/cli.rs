use clap::{Parser};
use clap::error::ErrorKind;
use crate::calculator::Calculator;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    modend1: String,

    #[arg()]
    modend2: String,
}

pub fn start() -> Result<(), clap::error::Error> {
    let args = Args::parse();

    let calculator = Calculator::from_str(args.modend1, args.modend2)
        .map_err(|_| clap::error::Error::new(ErrorKind::InvalidValue))?;

    println!("{}", calculator.calc());

    Ok(())
}