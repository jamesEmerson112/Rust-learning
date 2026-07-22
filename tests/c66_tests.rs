#[path = "../src/bin/c66_exercise.rs"]
#[allow(dead_code)]
mod c66_exercise;

use c66_exercise::crew_estimates;

#[test]
fn three_crew_members_tally_the_same_map() {
    // 3 threads × (4000 + 6500 + 3500 = 14000) = 42000
    assert_eq!(crew_estimates(), 42000);
}
