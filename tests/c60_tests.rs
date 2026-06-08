#[path = "../src/bin/c60_exercise.rs"]
#[allow(dead_code)]
mod c60_exercise;

use c60_exercise::closing_order;

#[test]
fn drops_in_reverse_order() {
    assert_eq!(
        closing_order(),
        vec!["closing B".to_string(), "closing A".to_string()]
    );
}
