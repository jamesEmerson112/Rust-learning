#[path = "../src/bin/c15_exercise.rs"]
#[allow(dead_code)]
mod c15_exercise;

use c15_exercise::normalize_word;

#[test]
fn lowercases() {
    assert_eq!(normalize_word("HELLO"), "hello");
}

#[test]
fn trims_punctuation() {
    assert_eq!(normalize_word("hello,"), "hello");
    assert_eq!(normalize_word("...rust!!!"), "rust");
}

#[test]
fn preserves_alphanumeric_mid_word() {
    assert_eq!(normalize_word("42nd"), "42nd");
}

#[test]
fn all_punctuation_becomes_empty() {
    assert_eq!(normalize_word("???"), "");
}
