#[path = "../src/bin/c64_exercise.rs"]
#[allow(dead_code)]
mod c64_exercise;

use c64_exercise::{borrow_after_drop_ok, double_borrow_fails};

#[test]
fn second_mut_borrow_is_rejected() {
    assert!(double_borrow_fails());
}

#[test]
fn borrow_succeeds_after_release() {
    assert!(borrow_after_drop_ok());
}
