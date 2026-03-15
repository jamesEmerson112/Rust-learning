#[path = "../src/bin/c13_exercise.rs"]
#[allow(dead_code)]
mod c13_exercise;

use c13_exercise::count_chars;

#[test]
fn count_chars_basic() {
    let counts = count_chars("aab");
    assert_eq!(counts.get(&'a'), Some(&2));
    assert_eq!(counts.get(&'b'), Some(&1));
}

#[test]
fn count_chars_empty() {
    let counts = count_chars("");
    assert!(counts.is_empty());
}

#[test]
fn count_chars_repeated() {
    let counts = count_chars("aaaa");
    assert_eq!(counts.get(&'a'), Some(&4));
    assert_eq!(counts.len(), 1);
}
