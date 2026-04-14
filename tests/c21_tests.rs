#[path = "../src/bin/c21_exercise.rs"]
#[allow(dead_code)]
mod c21_exercise;

use c21_exercise::squared_evens;

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

#[test]
fn all_evens() {
    assert_eq!(squared_evens(&[2, 4, 6]), vec![4, 16, 36]);
}
