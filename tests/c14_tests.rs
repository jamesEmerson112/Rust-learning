#[path = "../src/bin/c14_exercise.rs"]
#[allow(dead_code)]
mod c14_exercise;

use c14_exercise::count_numbers;

#[test]
fn counts_single_value() {
    let counts = count_numbers(&[7, 7, 7]);
    assert_eq!(counts.get(&7), Some(&3));
}

#[test]
fn counts_multiple_values() {
    let counts = count_numbers(&[1, 2, 2, 3, 3, 3]);
    assert_eq!(counts.get(&1), Some(&1));
    assert_eq!(counts.get(&2), Some(&2));
    assert_eq!(counts.get(&3), Some(&3));
}

#[test]
fn empty_input_returns_empty_map() {
    let counts = count_numbers(&[]);
    assert!(counts.is_empty());
}
