#[path = "../src/bin/c22_exercise.rs"]
#[allow(dead_code)]
mod c22_exercise;

use c22_exercise::squared_evens;

#[test]
fn squares_evens_only() {
    assert_eq!(squared_evens(&[1, 2, 3, 4]), vec![4, 16]);
}

#[test]
fn empty_input_yields_empty() {
    assert_eq!(squared_evens(&[]), Vec::<i32>::new());
}

#[test]
fn no_evens_yields_empty() {
    assert_eq!(squared_evens(&[1, 3, 5, 7]), Vec::<i32>::new());
}
