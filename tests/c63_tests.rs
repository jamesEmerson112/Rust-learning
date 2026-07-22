#[path = "../src/bin/c63_exercise.rs"]
#[allow(dead_code)]
mod c63_exercise;

use c63_exercise::SignalJammer;

#[test]
fn reload_returns_the_spent_cell() {
    let jammer = SignalJammer::new(10);
    assert_eq!(jammer.reload(99), 10);
    assert_eq!(jammer.charge_level(), 99);
}

#[test]
fn discharge_dumps_everything() {
    let jammer = SignalJammer::new(42);
    assert_eq!(jammer.discharge(), 42);
    assert_eq!(jammer.charge_level(), 0);
}

#[test]
fn all_through_a_shared_reference() {
    let jammer = SignalJammer::new(5);
    let alias: &SignalJammer = &jammer; // no &mut anywhere in this test
    alias.reload(7);
    assert_eq!(jammer.charge_level(), 7);
}
