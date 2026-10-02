#[path = "../src/bin/c57_exercise.rs"]
#[allow(dead_code)]
mod c57_exercise;

use c57_exercise::{BloodOxygen, BloodPressure, HeartRate, Sensor, full_monitor, over_budget, total_draw};

#[test]
fn individual_sensors() {
    assert_eq!(HeartRate.name(), "Heart Rate");
    assert_eq!(HeartRate.power_draw(), 40);
    assert_eq!(BloodOxygen.name(), "Blood Oxygen");
    assert_eq!(BloodOxygen.power_draw(), 25);
    assert_eq!(BloodPressure.name(), "Blood Pressure");
    assert_eq!(BloodPressure.power_draw(), 15);
}

#[test]
fn full_monitor_has_three_sensors() {
    let monitor = full_monitor();
    assert_eq!(monitor.len(), 3);
    let names: Vec<String> = monitor.iter().map(|s| s.name()).collect();
    assert_eq!(names, vec!["Heart Rate", "Blood Oxygen", "Blood Pressure"]);
}

#[test]
fn mixed_rack_total_draw() {
    assert_eq!(total_draw(&full_monitor()), 80);
}

#[test]
fn empty_monitor_draws_nothing() {
    let empty: Vec<Box<dyn Sensor>> = vec![];
    assert_eq!(total_draw(&empty), 0);
}

#[test]
fn budget_check() {
    let monitor = full_monitor();
    assert!(over_budget(&monitor, 60));
    assert!(!over_budget(&monitor, 100));
    assert!(!over_budget(&monitor, 80)); // exactly at budget is NOT over
}
