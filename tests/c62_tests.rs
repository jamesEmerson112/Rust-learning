#[path = "../src/bin/c62_exercise.rs"]
#[allow(dead_code)]
mod c62_exercise;

use c62_exercise::reverse_packet;

#[test]
fn reverses_even_length_packet() {
    let mut buf = vec![1, 2, 3, 4];
    reverse_packet(&mut buf);
    assert_eq!(buf, vec![4, 3, 2, 1]);
}

#[test]
fn reverses_odd_length_packet() {
    let mut buf = vec![1, 2, 3, 4, 5];
    reverse_packet(&mut buf);
    assert_eq!(buf, vec![5, 4, 3, 2, 1]);
}

#[test]
fn empty_packet_stays_empty() {
    let mut buf: Vec<i32> = vec![];
    reverse_packet(&mut buf);
    assert_eq!(buf, Vec::<i32>::new());
}
