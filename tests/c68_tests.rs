#[path = "../src/bin/c68_exercise.rs"]
#[allow(dead_code)]
mod c68_exercise;

use c68_exercise::free_beds_seen;

#[test]
fn nurses_see_the_charge_nurses_update() {
    assert_eq!(free_beds_seen(), 2);
}
