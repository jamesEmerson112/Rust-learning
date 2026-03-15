#[path = "../src/bin/c12_exercise.rs"]
#[allow(dead_code)]
mod c12_exercise;

use c12_exercise::safe_get;

#[test]
fn safe_get_valid_index() {
    assert_eq!(safe_get(&[10, 20, 30], 1), Some(20));
}

#[test]
fn safe_get_out_of_bounds() {
    assert_eq!(safe_get(&[10, 20, 30], 5), None);
}

#[test]
fn safe_get_empty_slice() {
    assert_eq!(safe_get(&[], 0), None);
}
