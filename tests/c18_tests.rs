#[path = "../src/bin/c18_exercise.rs"]
#[allow(dead_code)]
mod c18_exercise;

use c18_exercise::parse_pair;

#[test]
fn parse_pair_both_valid() {
    assert_eq!(parse_pair("3", "4"), Ok((3, 4)));
}

#[test]
fn parse_pair_first_invalid() {
    assert!(parse_pair("x", "4").is_err());
}

#[test]
fn parse_pair_second_invalid() {
    assert!(parse_pair("3", "y").is_err());
}

#[test]
fn parse_pair_negative_numbers() {
    assert_eq!(parse_pair("-5", "10"), Ok((-5, 10)));
}
