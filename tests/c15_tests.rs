#[path = "../src/bin/c15_exercise.rs"]
#[allow(dead_code)]
mod c15_exercise;

use c15_exercise::parse_age;

#[test]
fn parse_age_valid() {
    assert_eq!(parse_age("25"), Ok(25));
}

#[test]
fn parse_age_invalid() {
    assert!(parse_age("abc").is_err());
}

#[test]
fn parse_age_overflow() {
    assert!(parse_age("300").is_err());
}
