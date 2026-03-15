fn parse_age(input: &str) -> Result<u8, String> {
    let age: u8 = input
        .trim()
        .parse()
        .map_err(|_| format!("invalid age: {input}"))?;
    Ok(age)
}

fn main() {
    for input in ["25", "abc", "300"] {
        match parse_age(input) {
            Ok(age) => println!("Parsed age: {age}"),
            Err(e) => println!("Error: {e}"),
        }
    }
}
