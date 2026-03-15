#[path = "../src/bin/c02_exercise.rs"]
#[allow(dead_code)]
mod c02_exercise;

use c02_exercise::build_greeting;

#[test]
fn greeting_matches_expected_sentence() {
    let actual = build_greeting("Sam", 20);
    assert_eq!(actual, "Hello, Sam! You are 20 years old.");
}

#[test]
fn greeting_handles_empty_name() {
    let actual = build_greeting("", 1);
    assert_eq!(actual, "Hello, ! You are 1 years old.");
}
