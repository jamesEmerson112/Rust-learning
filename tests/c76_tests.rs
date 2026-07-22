#[path = "../src/bin/c76_exercise.rs"]
#[allow(dead_code)]
mod c76_exercise;

use c76_exercise::*;

#[test]
fn sums_a_clean_price_list() {
    let rows = ["Gel Manicure,4500", "Pedicure,3500", "Acrylic Full Set,6000"];
    assert_eq!(daily_total(&rows), Ok(14000));
}

#[test]
fn corrupt_price_is_reported_as_error() {
    let rows = ["Gel Manicure,4500", "Pedicure,oops", "Acrylic Full Set,6000"];
    assert!(daily_total(&rows).is_err());
}

#[test]
fn empty_list_totals_zero() {
    let rows: [&str; 0] = [];
    assert_eq!(daily_total(&rows), Ok(0));
}
