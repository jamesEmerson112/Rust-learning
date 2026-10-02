#[path = "../src/bin/c63_exercise.rs"]
#[allow(dead_code)]
mod c63_exercise;

use c63_exercise::Defibrillator;

#[test]
fn recharge_returns_the_previous_charge() {
    let defib = Defibrillator::new(150);
    assert_eq!(defib.recharge(200), 150);
    assert_eq!(defib.charge_level(), 200);
}

#[test]
fn shock_delivers_everything() {
    let defib = Defibrillator::new(200);
    assert_eq!(defib.shock(), 200);
    assert_eq!(defib.charge_level(), 0);
}

#[test]
fn all_through_a_shared_reference() {
    let defib = Defibrillator::new(120);
    let alias: &Defibrillator = &defib; // no &mut anywhere in this test
    alias.recharge(150);
    assert_eq!(defib.charge_level(), 150);
}
