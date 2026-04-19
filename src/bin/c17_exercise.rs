use std::fmt::format;

pub fn parse_age(input: &str) -> Result<u8, String> {
    // TODO: Use `match` on `input.trim().parse::<u8>()`:
    //   Ok(n)  => Ok(n)
    //   Err(_) => Err(format!("invalid age: {input}"))
    match input.trim().parse::<u8>() {
        Ok(n) => Ok(n),
        Err(_)    => Err(format!("Invalid age: {input}")),
    }
}

fn main() {
    println!("{:?}", parse_age("25"));
    println!("{:?}", parse_age("abc"));
}
