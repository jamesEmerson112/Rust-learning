#[path = "../src/bin/c14_exercise.rs"]
#[allow(dead_code)]
mod c14_exercise;

use c14_exercise::word_count;

#[test]
fn counts_words_case_insensitive() {
    let counts = word_count("Rust rust RUST");
    assert_eq!(counts.get("rust"), Some(&3));
}

#[test]
fn trims_basic_punctuation() {
    let counts = word_count("hello, world! hello.");
    assert_eq!(counts.get("hello"), Some(&2));
    assert_eq!(counts.get("world"), Some(&1));
}

#[test]
fn empty_input_returns_empty_map() {
    let counts = word_count("");
    assert!(counts.is_empty());
}
