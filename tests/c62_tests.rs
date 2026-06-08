#[path = "../src/bin/c62_exercise.rs"]
#[allow(dead_code)]
mod c62_exercise;

use c62_exercise::reverse_in_place;

#[test]
fn reverses_even_length() {
    let mut v = vec![1, 2, 3, 4];
    reverse_in_place(&mut v);
    assert_eq!(v, vec![4, 3, 2, 1]);
}

#[test]
fn reverses_odd_length() {
    let mut v = vec![1, 2, 3, 4, 5];
    reverse_in_place(&mut v);
    assert_eq!(v, vec![5, 4, 3, 2, 1]);
}

#[test]
fn empty_stays_empty() {
    let mut v: Vec<i32> = vec![];
    reverse_in_place(&mut v);
    assert_eq!(v, Vec::<i32>::new());
}
