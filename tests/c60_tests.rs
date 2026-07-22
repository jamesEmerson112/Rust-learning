#[path = "../src/bin/c60_exercise.rs"]
#[allow(dead_code)]
mod c60_exercise;

use c60_exercise::{burn_order, early_burn};

#[test]
fn traces_burn_in_reverse_order() {
    assert_eq!(
        burn_order(),
        vec!["bravo trace burned".to_string(), "alpha trace burned".to_string()]
    );
}

#[test]
fn compromised_uplink_burns_first() {
    assert_eq!(
        early_burn(),
        vec!["alpha trace burned".to_string(), "bravo trace burned".to_string()]
    );
}
