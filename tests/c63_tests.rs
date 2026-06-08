#[path = "../src/bin/c63_exercise.rs"]
#[allow(dead_code)]
mod c63_exercise;

use c63_exercise::Counter;

#[test]
fn replace_returns_old() {
    let c = Counter::new(10);
    assert_eq!(c.replace_with(99), 10);
    assert_eq!(c.get(), 99);
}

#[test]
fn take_resets_to_zero() {
    let c = Counter::new(42);
    assert_eq!(c.take(), 42);
    assert_eq!(c.get(), 0);
}
