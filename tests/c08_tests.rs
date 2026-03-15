#[path = "../src/bin/c08_exercise.rs"]
#[allow(dead_code)]
mod c08_exercise;

use c08_exercise::first_word_len;

#[test]
fn first_word_len_basic() {
    assert_eq!(first_word_len("rust ownership"), 4);
}

#[test]
fn first_word_len_single_word() {
    assert_eq!(first_word_len("borrow"), 6);
}

#[test]
fn first_word_len_empty_string() {
    assert_eq!(first_word_len(""), 0);
}

#[test]
fn first_word_len_ignores_leading_spaces() {
    assert_eq!(first_word_len("   leading spaces"), 7);
}
