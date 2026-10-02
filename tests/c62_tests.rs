#[path = "../src/bin/c62_exercise.rs"]
#[allow(dead_code)]
mod c62_exercise;

use c62_exercise::retrace;

#[test]
fn reverses_even_length_path() {
    let mut path = vec![1, 2, 3, 4];
    retrace(&mut path);
    assert_eq!(path, vec![4, 3, 2, 1]);
}

#[test]
fn reverses_odd_length_path() {
    let mut path = vec![1, 2, 3, 4, 5];
    retrace(&mut path);
    assert_eq!(path, vec![5, 4, 3, 2, 1]);
}

#[test]
fn empty_path_stays_empty() {
    let mut path: Vec<i32> = vec![];
    retrace(&mut path);
    assert_eq!(path, Vec::<i32>::new());
}
