pub fn safe_divide(a: &str, b: &str) -> Result<f64, String> {
    // TODO:
    // 1) Parse a as f64, else Err("invalid number for a")
    // 2) Parse b as f64, else Err("invalid number for b")
    // 3) If b == 0.0, Err("cannot divide by zero")
    // 4) Otherwise return Ok(a / b)
    let left = a.parse::<f64>().map_err(|_| "invalid number a".to_string())?;
    let right = b.parse::<f64>().map_err(|_| "invalid number b".to_string())?;

    Ok(left / right)
}

fn main() {
    println!("{:?}", safe_divide("10", "2"));
    println!("{:?}", safe_divide("12", "5"));

}
