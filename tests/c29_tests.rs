#[path = "../src/bin/c29_exercise.rs"]
#[allow(dead_code)]
mod c29_exercise;

use c29_exercise::Counter;

#[test]
fn new_counter_is_zero() {
    let c = Counter::new();
    assert_eq!(c.value(), 0);
}

#[test]
fn increment_works() {
    let mut c = Counter::new();
    c.increment();
    c.increment();
    assert_eq!(c.value(), 2);
}

#[test]
fn independent_instances() {
    let mut a = Counter::new();
    let mut b = Counter::new();
    a.increment();
    a.increment();
    b.increment();
    assert_eq!(a.value(), 2);
    assert_eq!(b.value(), 1);
}
