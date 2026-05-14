#[path = "../src/bin/c45_exercise.rs"]
#[allow(dead_code)]
mod c45_exercise;

use c45_exercise::WalkInQueue;

#[test]
fn yields_all_names() {
    let queue = WalkInQueue::new(vec![
        "Mai".to_string(),
        "Linh".to_string(),
        "Trang".to_string(),
    ]);
    let names: Vec<String> = queue.collect();
    assert_eq!(names, vec!["Mai", "Linh", "Trang"]);
}

#[test]
fn empty_queue() {
    let queue = WalkInQueue::new(vec![]);
    let names: Vec<String> = queue.collect();
    assert!(names.is_empty());
}

#[test]
fn single_client() {
    let queue = WalkInQueue::new(vec!["Mai".to_string()]);
    let names: Vec<String> = queue.collect();
    assert_eq!(names, vec!["Mai"]);
}
