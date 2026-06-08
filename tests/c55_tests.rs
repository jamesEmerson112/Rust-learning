#[path = "../src/bin/c55_exercise.rs"]
#[allow(dead_code)]
mod c55_exercise;

use c55_exercise::fib;

#[test]
fn base_cases() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
}

#[test]
fn small_values() {
    assert_eq!(fib(2), 1);
    assert_eq!(fib(7), 13);
    assert_eq!(fib(10), 55);
}
