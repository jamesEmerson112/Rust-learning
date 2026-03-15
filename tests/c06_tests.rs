#[path = "../src/bin/c06_exercise.rs"]
#[allow(dead_code)]
mod c06_exercise;

use c06_exercise::fizzbuzz;

#[test]
fn fizzbuzz_first_five_items() {
    let actual = fizzbuzz(5);
    let expected = vec!["1", "2", "Fizz", "4", "Buzz"];
    assert_eq!(actual, expected);
}

#[test]
fn fizzbuzz_includes_fifteen_rule() {
    let actual = fizzbuzz(15);
    assert_eq!(actual[14], "FizzBuzz");
}

#[test]
fn fizzbuzz_zero_is_empty() {
    let actual = fizzbuzz(0);
    assert!(actual.is_empty());
}
