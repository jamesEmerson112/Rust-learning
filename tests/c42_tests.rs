#[path = "../src/bin/c42_exercise.rs"]
#[allow(dead_code)]
mod c42_exercise;

use c42_exercise::TipJar;

#[test]
fn starts_empty() {
    let jar = TipJar::new();
    assert_eq!(jar.total(), 0);
}

#[test]
fn add_tips() {
    let jar = TipJar::new();
    jar.add(500);
    jar.add(300);
    assert_eq!(jar.total(), 800);
}

#[test]
fn immutable_reference() {
    let jar = TipJar::new();
    jar.add(100);
    jar.add(200);
    jar.add(300);
    assert_eq!(jar.total(), 600);
}
