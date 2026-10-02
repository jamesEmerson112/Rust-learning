#[path = "../src/bin/c76_exercise.rs"]
#[allow(dead_code)]
mod c76_exercise;

use c76_exercise::*;

#[test]
fn sums_a_clean_fluid_chart() {
    let rows = ["IV saline,500", "Water,250", "Soup,300"];
    assert_eq!(fluid_intake(&rows), Ok(1050));
}

#[test]
fn corrupt_amount_is_reported_as_error() {
    let rows = ["IV saline,500", "Water,oops", "Soup,300"];
    assert!(fluid_intake(&rows).is_err());
}

#[test]
fn empty_chart_totals_zero() {
    let rows: [&str; 0] = [];
    assert_eq!(fluid_intake(&rows), Ok(0));
}
