#[path = "../src/bin/c65_exercise.rs"]
#[allow(dead_code)]
mod c65_exercise;

use c65_exercise::double_delivery;

#[test]
fn catches_a_double_delivery() {
    assert!(double_delivery(&[7011, 7012, 7013, 7011]));
}

#[test]
fn clean_log_passes() {
    assert!(!double_delivery(&[7011, 7012, 7013, 7014]));
}

#[test]
fn empty_log_is_clean() {
    assert!(!double_delivery(&[]));
}
