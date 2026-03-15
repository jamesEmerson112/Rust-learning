#[path = "../src/bin/c05_exercise.rs"]
#[allow(dead_code)]
mod c05_exercise;

use c05_exercise::evens_up_to;

#[test]
fn evens_up_to_six() {
    assert_eq!(evens_up_to(6), vec![2, 4, 6]);
}

#[test]
fn evens_up_to_zero() {
    assert_eq!(evens_up_to(0), Vec::<u32>::new());
}

#[test]
fn evens_up_to_one() {
    assert_eq!(evens_up_to(1), Vec::<u32>::new());
}
