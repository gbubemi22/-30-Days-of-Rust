fn parse_score(text: &str) -> Result<u32, std::num::ParseIntError> {
    let score = text.trim().parse::<u32>()?;
    Ok(score)
}

fn main() {
    for input in ["87", "not a score"] {
        match parse_score(input) {
            Ok(score) => println!("Score: {score}"),
            Err(error) => println!("Invalid score '{input}': {error}"),
        }
    }
}
