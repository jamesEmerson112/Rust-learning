#[path = "../src/bin/c64_exercise.rs"]
#[allow(dead_code)]
mod c64_exercise;

use c64_exercise::Chart;

#[test]
fn note_during_review_is_refused_not_a_crash() {
    let chart = Chart::new();
    chart.add_note("BP 120/80 at 08:00");
    let result = chart.note_during_review();
    assert!(result.is_err(), "a note during a review must be refused (Err), not panic");
    assert_eq!(chart.note_count(), 1, "nothing may be written while the review is open");
}

#[test]
fn notes_flow_after_the_review_releases() {
    let chart = Chart::new();
    chart.add_note("BP 120/80 at 08:00");
    assert_eq!(chart.note_after_review(), 2);
}
