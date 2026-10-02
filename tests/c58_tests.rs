#[path = "../src/bin/c58_exercise.rs"]
#[allow(dead_code)]
mod c58_exercise;

use c58_exercise::{Pump, two_hour_volume};

#[test]
fn programmed_rate_reaches_the_reader() {
    let iv = Pump::new("IV pump 3", 21);
    assert!(!iv.is_default(), "21 ml/h is programmed — the pump is not on its factory default");
    // deref coercion: &Pump<i32> -> &i32
    assert_eq!(two_hour_volume(&iv), 42);
}

#[test]
fn auto_deref_reaches_string_methods() {
    let syringe = Pump::new("syringe pump 1", String::from("insulin"));
    assert_eq!(syringe.len(), 7);
    assert!(syringe.starts_with("ins"));
}

#[test]
fn explicit_star_deref() {
    let feed = Pump::new("feeding pump", 9);
    assert_eq!(*feed + 1, 10);
}
