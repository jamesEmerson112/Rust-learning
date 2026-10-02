#[path = "../src/bin/c75_exercise.rs"]
#[allow(dead_code)]
mod c75_exercise;

use c75_exercise::*;

fn sample_shift() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Mai", "Mr. Hung"),
        ("Mai", "Mrs. Lan"),
        ("Mai", "Mr. Bao"),
        ("Mai", "Mr. Hung"),
        ("Mai", "Mrs. Lan"),
        ("Mai", "Mr. Hung"),
        ("Linh", "Mrs. Lan"),
        ("Linh", "Mr. Hung"),
        ("Trang", "Mr. Bao"),
    ]
}

#[test]
fn counts_accumulate_per_nurse() {
    let counts = visit_counts(&sample_shift());
    assert_eq!(counts.get("Mai"), Some(&6));
    assert_eq!(counts.get("Linh"), Some(&2));
    assert_eq!(counts.get("Trang"), Some(&1));
}

#[test]
fn unknown_nurse_is_absent() {
    let counts = visit_counts(&sample_shift());
    assert_eq!(counts.get("Kim"), None);
}

#[test]
fn empty_shift_is_empty_map() {
    let counts = visit_counts(&[]);
    assert!(counts.is_empty());
}
