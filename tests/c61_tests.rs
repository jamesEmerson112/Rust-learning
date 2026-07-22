#[path = "../src/bin/c61_exercise.rs"]
#[allow(dead_code)]
mod c61_exercise;

use c61_exercise::{link_counts, trace_lost};

#[test]
fn daemon_has_no_strong_grip() {
    // one strong owner (you), one weak watcher (the daemon)
    assert_eq!(link_counts(), (1, 1));
}

#[test]
fn jack_out_loses_the_trace() {
    assert!(trace_lost(), "after dropping the session, the daemon must not reach it");
}
