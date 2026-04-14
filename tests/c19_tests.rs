#[path = "../src/bin/c19_exercise.rs"]
#[allow(dead_code)]
mod c19_exercise;

use c19_exercise::safe_divide;

#[test]
fn safe_divide_success() {
    let value = safe_divide("10", "4").expect("expected successful division");
    assert!((value - 2.5).abs() < 1e-9);
}

#[test]
fn safe_divide_invalid_a() {
    assert_eq!(
        safe_divide("x", "4"),
        Err("invalid number for a".to_string())
    );
}

#[test]
fn safe_divide_invalid_b() {
    assert_eq!(
        safe_divide("4", "y"),
        Err("invalid number for b".to_string())
    );
}

#[test]
fn safe_divide_divide_by_zero() {
    assert_eq!(
        safe_divide("4", "0"),
        Err("cannot divide by zero".to_string())
    );
}
