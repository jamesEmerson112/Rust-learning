#[path = "../src/bin/c26_exercise.rs"]
#[allow(dead_code)]
mod c26_exercise;

use c26_exercise::longer;

#[test]
fn picks_longer() {
    assert_eq!(longer("short", "a longer string"), "a longer string");
}

#[test]
fn picks_first_on_tie() {
    assert_eq!(longer("same", "size"), "same");
}

#[test]
fn works_with_empty() {
    assert_eq!(longer("", "hello"), "hello");
}
