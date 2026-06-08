#[path = "../src/bin/c56_exercise.rs"]
#[allow(dead_code)]
mod c56_exercise;

use c56_exercise::{Waitlist, build_waitlist, waitlist_len};

#[test]
fn empty_is_zero() {
    assert_eq!(waitlist_len(&Waitlist::Empty), 0);
}

#[test]
fn counts_clients() {
    let list = build_waitlist(&["Mai", "Linh", "Trang"]);
    assert_eq!(waitlist_len(&list), 3);
}

#[test]
fn single_client() {
    let list = build_waitlist(&["Mai"]);
    assert_eq!(waitlist_len(&list), 1);
}
