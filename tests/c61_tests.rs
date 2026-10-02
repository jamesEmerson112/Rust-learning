#[path = "../src/bin/c61_exercise.rs"]
#[allow(dead_code)]
mod c61_exercise;

use c61_exercise::{alarm_silent_after_discharge, link_counts};

#[test]
fn alarm_has_no_strong_grip() {
    // one strong owner (the ward), one weak watcher (the alarm)
    assert_eq!(link_counts(), (1, 1));
}

#[test]
fn discharge_silences_the_alarm() {
    assert!(
        alarm_silent_after_discharge(),
        "after discharge, the alarm must not reach the patient"
    );
}
