use std::error::Error;
use std::io::{self, Read};
use std::process::ExitCode;

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("greet") => {
            let name = args.next().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "usage: word-tool greet NAME")
            })?;
            println!("Hello, {name}!");
        }
        Some("count") => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            println!("Words: {}", input.split_whitespace().count());
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "usage: word-tool greet NAME | count",
            )
            .into());
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
