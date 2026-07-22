#[path = "../src/bin/c64_exercise.rs"]
#[allow(dead_code)]
mod c64_exercise;

use c64_exercise::IntrusionLog;

#[test]
fn contended_write_is_refused_not_a_crash() {
    let log = IntrusionLog::new();
    log.record("breach at relay-7");
    let result = log.record_during_sweep();
    assert!(result.is_err(), "a write during a sweep must be refused (Err), not panic");
    assert_eq!(log.entry_count(), 1, "nothing may be written while the sweep is active");
}

#[test]
fn writes_flow_after_the_sweep_releases() {
    let log = IntrusionLog::new();
    log.record("breach at relay-7");
    assert_eq!(log.record_after_sweep(), 2);
}
