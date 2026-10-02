#[path = "../src/bin/c54_exercise.rs"]
#[allow(dead_code)]
mod c54_exercise;

use c54_exercise::Ward;

#[test]
fn log_and_list() {
    let mut ward = Ward::new();
    ward.log_visit("Mai", "Mr. Hung", 45);
    ward.log_visit("Linh", "Mrs. Lan", 35);
    assert_eq!(ward.list().len(), 2);
    assert_eq!(ward.list()[0].0, "Mai");
    assert_eq!(ward.list()[1].2, 35);
}

#[test]
fn care_minutes_by_nurse() {
    let mut ward = Ward::new();
    ward.log_visit("Mai", "Mr. Hung", 45);
    ward.log_visit("Linh", "Mrs. Lan", 35);
    ward.log_visit("Mai", "Mr. Bao", 30);

    let care = ward.minutes_by_nurse();
    assert_eq!(care.get("Mai"), Some(&75));
    assert_eq!(care.get("Linh"), Some(&35));
}

#[test]
fn empty_ward() {
    let ward = Ward::new();
    assert!(ward.list().is_empty());
    assert!(ward.minutes_by_nurse().is_empty());
}
