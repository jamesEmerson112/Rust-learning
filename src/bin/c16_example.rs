fn safe_divide(a: &str, b: &str) -> Result<f64, String> {
    let left = a
        .parse::<f64>()
        .map_err(|_| "invalid number for a".to_string())?;
    let right = b
        .parse::<f64>()
        .map_err(|_| "invalid number for b".to_string())?;

    if right == 0.0 {
        return Err("cannot divide by zero".to_string());
    }

    Ok(left / right)
}

fn main() {
    for (a, b) in [("10", "2"), ("7", "0"), ("x", "4")] {
        match safe_divide(a, b) {
            Ok(value) => println!("{a} / {b} = {value:.2}"),
            Err(err) => println!("Error for ({a}, {b}): {err}"),
        }
    }
}
