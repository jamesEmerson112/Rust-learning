pub fn safe_divide(a: &str, b: &str) -> Result<f64, String> {
    // TODO:
    // 1) Parse a as f64, else Err("invalid number for a")
    // 2) Parse b as f64, else Err("invalid number for b")
    // 3) If b == 0.0, Err("cannot divide by zero")
    // 4) Otherwise return Ok(a / b)
    let _ = (a, b);
    Err("TODO: implement safe_divide".to_string())
}

fn main() {
    println!("{:?}", safe_divide("10", "2"));
}
