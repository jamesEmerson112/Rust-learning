use std::num::ParseIntError;

pub fn parse_pair(a: &str, b: &str) -> Result<(i32, i32), ParseIntError> {
    // TODO: Parse both strings as i32 using `?` to early-return on failure.
    // Return Ok((x, y)) when both parses succeed.
    let x:i32 = a.parse::<i32>()?;
    let y:i32 = b.parse::<i32>()?;
    Ok((x, y))
}

fn main() {
    println!("{:?}", parse_pair("3", "4"));
    println!("{:?}", parse_pair("3", "x"));
}
