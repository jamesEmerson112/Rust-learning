#[path = "../src/bin/c58_exercise.rs"]
#[allow(dead_code)]
mod c58_exercise;

use c58_exercise::{Implant, boost};

#[test]
fn installed_firmware_shines_through() {
    let reflex = Implant::new("neural-lace", 21);
    assert!(!reflex.is_stock(), "firmware 21 is installed — chrome is not stock");
    // deref coercion: &Implant<i32> -> &i32
    assert_eq!(boost(&reflex), 42);
}

#[test]
fn auto_deref_reaches_string_methods() {
    let stealth = Implant::new("ghostweave", String::from("stealth"));
    assert_eq!(stealth.len(), 7);
    assert!(stealth.starts_with("ste"));
}

#[test]
fn explicit_star_deref() {
    let optic = Implant::new("optic", 9);
    assert_eq!(*optic + 1, 10);
}
