#[path = "../src/bin/c25_exercise.rs"]
#[allow(dead_code)]
mod c25_exercise;

use c25_exercise::largest;

#[test]
fn largest_on_integers() {
    assert_eq!(largest(&[3, 9, 2, 14, 5]), Some(14));
}

#[test]
fn largest_on_chars() {
    assert_eq!(largest(&['a', 'z', 'm']), Some('z'));
}

#[test]
fn largest_on_empty_slice() {
    let empty: [i32; 0] = [];
    assert_eq!(largest(&empty), None);
}
