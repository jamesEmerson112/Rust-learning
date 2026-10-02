// BUG: Mai is sure she made 6 patient visits this shift, but the care log insists she made
// exactly 1. So does Linh. So does Trang. Every nurse's tally is stuck at 1 no matter how
// busy the shift was. The code compiles and runs fine — it just can't count. Find and fix it.
// (This drills c13/c14: HashMap counting. The tests in tests/c75_tests.rs must go green.)
use std::collections::HashMap;

pub fn visit_counts(entries: &[(&str, &str)]) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for &(nurse, _patient) in entries {
        counts.insert(nurse.to_string(), 1);
    }
    counts
}

fn main() {
    let shift = [
        ("Mai", "Mr. Hung"),
        ("Mai", "Mrs. Lan"),
        ("Mai", "Mr. Bao"),
        ("Mai", "Mr. Hung"),
        ("Mai", "Mrs. Lan"),
        ("Mai", "Mr. Hung"),
        ("Linh", "Mrs. Lan"),
        ("Linh", "Mr. Hung"),
        ("Trang", "Mr. Bao"),
    ];
    let counts = visit_counts(&shift);
    println!("Mai made {:?} visits this shift", counts.get("Mai"));
    println!("Linh made {:?} visits this shift", counts.get("Linh"));
    println!("Trang made {:?} visits this shift", counts.get("Trang"));
}
