#[path = "../src/bin/c30_exercise.rs"]
#[allow(dead_code)]
mod c30_exercise;

use c30_exercise::boxed_price;

#[test]
fn boxed_holds_price() {
    let b = boxed_price(4500);
    assert_eq!(*b, 4500);
}

#[test]
fn boxed_handles_zero() {
    let b = boxed_price(0);
    assert_eq!(*b, 0);
}

#[test]
fn boxed_handles_negative() {
    let b = boxed_price(-100);
    assert_eq!(*b, -100);
}
