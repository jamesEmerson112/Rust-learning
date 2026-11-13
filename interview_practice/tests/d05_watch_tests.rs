//! Tests for the digital watch in `src/bin/d05_watch.rs`.
//!
//! Contents:
//! - clock tests: the midnight start, carrying, wrapping and rejected times
//! - face test: the 12-hour display
//! - alarm tests: ringing once, clearing and rejected times
//! - stopwatch test: counting only while running

#[path = "../src/bin/d05_watch.rs"]
#[allow(dead_code)]
mod d05_watch;

use d05_watch::{HourFormat, Watch, WatchError};

/// Description: A new watch reads midnight on the 24-hour face.
/// Inputs: none
/// Outputs: none
#[test]
fn a_new_watch_reads_midnight() {
    let watch = Watch::new();
    assert_eq!(watch.time(), (0, 0, 0));
    assert_eq!(watch.to_string(), "00:00:00");
}

/// Description: One tick past 10:59:59 carries into the minutes and the hours.
/// Inputs: none
/// Outputs: none
#[test]
fn a_tick_carries_into_minutes_and_hours() {
    let mut watch = Watch::new();
    watch.set_time(10, 59, 59).unwrap();
    watch.tick();
    assert_eq!(watch.to_string(), "11:00:00");
}

/// Description: One tick past 23:59:59 wraps to midnight.
/// Inputs: none
/// Outputs: none
#[test]
fn the_clock_wraps_at_midnight() {
    let mut watch = Watch::new();
    watch.set_time(23, 59, 59).unwrap();
    watch.tick();
    assert_eq!(watch.to_string(), "00:00:00");
}

/// Description: An out-of-range hour, minute or second is rejected and leaves the clock alone.
/// Inputs: none
/// Outputs: none
#[test]
fn impossible_times_are_rejected_and_change_nothing() {
    let mut watch = Watch::new();
    watch.set_time(8, 30, 0).unwrap();
    assert_eq!(
        watch.set_time(24, 0, 0),
        Err(WatchError::InvalidTime { hour: 24, minute: 0, second: 0 })
    );
    assert!(watch.set_time(7, 60, 0).is_err());
    assert!(watch.set_time(7, 0, 60).is_err());
    assert_eq!(watch.time(), (8, 30, 0));
}

/// Description: The 12-hour face shows 12 for the midnight and noon hours and drops the leading
/// zero.
/// Inputs: none
/// Outputs: none
#[test]
fn the_12_hour_face() {
    let mut watch = Watch::new();
    watch.set_format(HourFormat::H12);
    for (hour, minute, second, face) in [
        (0, 5, 9, "12:05:09 AM"),
        (11, 59, 59, "11:59:59 AM"),
        (12, 0, 0, "12:00:00 PM"),
        (13, 30, 0, "1:30:00 PM"),
    ] {
        watch.set_time(hour, minute, second).unwrap();
        assert_eq!(watch.to_string(), face);
    }
}

/// Description: The alarm rings on the one tick that reaches 7:00 and on neither tick around it.
/// Inputs: none
/// Outputs: none
#[test]
fn the_alarm_rings_on_the_tick_that_reaches_it() {
    let mut watch = Watch::new();
    watch.set_time(6, 59, 58).unwrap();
    watch.set_alarm(7, 0).unwrap();
    assert!(!watch.tick());
    assert!(watch.tick());
    assert!(!watch.tick());
}

/// Description: A cleared alarm stays silent at its old time.
/// Inputs: none
/// Outputs: none
#[test]
fn a_cleared_alarm_never_rings() {
    let mut watch = Watch::new();
    watch.set_time(6, 59, 59).unwrap();
    watch.set_alarm(7, 0).unwrap();
    watch.clear_alarm();
    assert!(!watch.tick());
}

/// Description: An alarm at an impossible hour is rejected.
/// Inputs: none
/// Outputs: none
#[test]
fn an_impossible_alarm_is_rejected() {
    let mut watch = Watch::new();
    assert_eq!(
        watch.set_alarm(25, 0),
        Err(WatchError::InvalidTime { hour: 25, minute: 0, second: 0 })
    );
}

/// Description: The stopwatch counts only the ticks between start and stop, and reset zeroes it.
/// Inputs: none
/// Outputs: none
#[test]
fn the_stopwatch_counts_only_while_running() {
    let mut watch = Watch::new();
    for _ in 0..3 {
        watch.tick();
    }
    assert_eq!(watch.stopwatch_seconds(), 0);
    watch.start_stopwatch();
    for _ in 0..5 {
        watch.tick();
    }
    watch.stop_stopwatch();
    for _ in 0..2 {
        watch.tick();
    }
    assert_eq!(watch.stopwatch_seconds(), 5);
    watch.reset_stopwatch();
    assert_eq!(watch.stopwatch_seconds(), 0);
}
