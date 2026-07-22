#[path = "../src/bin/c75_exercise.rs"]
#[allow(dead_code)]
mod c75_exercise;

use c75_exercise::*;

fn sample_day() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Mai", "Gel Manicure"),
        ("Mai", "Pedicure"),
        ("Mai", "Acrylic Full Set"),
        ("Mai", "Gel Manicure"),
        ("Mai", "Pedicure"),
        ("Mai", "Gel Manicure"),
        ("Linh", "Pedicure"),
        ("Linh", "Gel Manicure"),
        ("Trang", "Acrylic Full Set"),
    ]
}

#[test]
fn counts_accumulate_per_technician() {
    let counts = service_counts(&sample_day());
    assert_eq!(counts.get("Mai"), Some(&6));
    assert_eq!(counts.get("Linh"), Some(&2));
    assert_eq!(counts.get("Trang"), Some(&1));
}

#[test]
fn unknown_technician_is_absent() {
    let counts = service_counts(&sample_day());
    assert_eq!(counts.get("Kim"), None);
}

#[test]
fn empty_day_is_empty_map() {
    let counts = service_counts(&[]);
    assert!(counts.is_empty());
}
