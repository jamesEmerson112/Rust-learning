pub fn parse_age(input: &str) -> Result<u8, String> {
    // TODO: Use `match` on `input.trim().parse::<u8>()`:
    //   Ok(n)  => Ok(n)
    //   Err(_) => Err(format!("invalid age: {input}"))
    let _ = input;
    Err("TODO: implement parse_age".to_string())
}

fn main() {
    println!("{:?}", parse_age("25"));
    println!("{:?}", parse_age("abc"));
}
