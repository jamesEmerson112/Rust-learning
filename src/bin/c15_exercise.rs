pub fn parse_age(input: &str) -> Result<u8, String> {
    // TODO: Parse the input string as a u8.
    // Return Ok(age) on success, or Err with a descriptive message on failure.
    let _ = input;
    Err("TODO: implement parse_age".to_string())
}

fn main() {
    println!("{:?}", parse_age("25"));
    println!("{:?}", parse_age("abc"));
}
