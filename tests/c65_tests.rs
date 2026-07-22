#[path = "../src/bin/c65_exercise.rs"]
#[allow(dead_code)]
mod c65_exercise;

use c65_exercise::signature_reused;

#[test]
fn detects_a_replayed_signature() {
    assert!(signature_reused(&[7011, 7012, 7013, 7011]));
}

#[test]
fn clean_batch_passes() {
    assert!(!signature_reused(&[7011, 7012, 7013, 7014]));
}

#[test]
fn empty_batch_is_clean() {
    assert!(!signature_reused(&[]));
}
