mod cli;
mod calculator;

fn main() {
    match cli::start() {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}
