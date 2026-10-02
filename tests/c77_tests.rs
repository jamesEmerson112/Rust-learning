#[path = "../src/bin/c77_exercise.rs"]
#[allow(dead_code)]
mod c77_exercise;

use c77_exercise::*;

#[test]
fn sums_only_critical_beds() {
    let ward = [
        ("Critical", 10u32),
        ("Stable", 2),
        ("Critical", 15),
        ("Stable", 3),
        ("Critical", 5),
    ];
    assert_eq!(critical_oxygen_total(&ward), 30);
}

#[test]
fn no_critical_beds_is_zero() {
    let ward = [("Stable", 2u32), ("Stable", 3)];
    assert_eq!(critical_oxygen_total(&ward), 0);
}

#[test]
fn empty_ward_is_zero() {
    let ward: [(&str, u32); 0] = [];
    assert_eq!(critical_oxygen_total(&ward), 0);
}
