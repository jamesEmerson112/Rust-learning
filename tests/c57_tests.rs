#[path = "../src/bin/c57_exercise.rs"]
#[allow(dead_code)]
mod c57_exercise;

use c57_exercise::{Ghost, Icepick, Program, Siphon, full_loadout, over_budget, total_draw};

#[test]
fn individual_programs() {
    assert_eq!(Icepick.name(), "Icepick");
    assert_eq!(Icepick.power_draw(), 40);
    assert_eq!(Siphon.name(), "Siphon");
    assert_eq!(Siphon.power_draw(), 25);
    assert_eq!(Ghost.name(), "Ghost");
    assert_eq!(Ghost.power_draw(), 15);
}

#[test]
fn full_loadout_is_three_programs() {
    let deck = full_loadout();
    assert_eq!(deck.len(), 3);
    let names: Vec<String> = deck.iter().map(|p| p.name()).collect();
    assert_eq!(names, vec!["Icepick", "Siphon", "Ghost"]);
}

#[test]
fn mixed_rack_total_draw() {
    assert_eq!(total_draw(&full_loadout()), 80);
}

#[test]
fn empty_rack_draws_nothing() {
    let empty: Vec<Box<dyn Program>> = vec![];
    assert_eq!(total_draw(&empty), 0);
}

#[test]
fn budget_check() {
    let deck = full_loadout();
    assert!(over_budget(&deck, 60));
    assert!(!over_budget(&deck, 100));
    assert!(!over_budget(&deck, 80)); // exactly at budget is NOT over
}
