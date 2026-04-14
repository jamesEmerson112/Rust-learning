use std::num::ParseIntError;

fn parse_pair(a: &str, b: &str) -> Result<(i32, i32), ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok((x, y))
}

fn main() {
    println!("{:?}", parse_pair("3", "4"));
    println!("{:?}", parse_pair("10", "x"));
}
