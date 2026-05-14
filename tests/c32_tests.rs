#[path = "../src/bin/c32_exercise.rs"]
#[allow(dead_code)]
mod c32_exercise;

use c32_exercise::ClientCounter;

#[test]
fn new_counter_is_zero() {
    let c = ClientCounter::new();
    assert_eq!(c.value(), 0);
}

#[test]
fn increment_works() {
    let mut c = ClientCounter::new();
    c.increment();
    c.increment();
    assert_eq!(c.value(), 2);
}

#[test]
fn independent_instances() {
    let mut a = ClientCounter::new();
    let mut b = ClientCounter::new();
    a.increment();
    a.increment();
    b.increment();
    assert_eq!(a.value(), 2);
    assert_eq!(b.value(), 1);
}
