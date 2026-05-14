#[path = "../src/bin/c48_exercise.rs"]
#[allow(dead_code)]
mod c48_exercise;

use c48_exercise::save_daily_log;

#[test]
fn writes_log_file() {
    let path = "test_daily_c48.txt";
    let entries = vec![
        ("Mai - Gel".to_string(), 4500u32),
        ("Linh - Pedicure".to_string(), 3500),
    ];
    save_daily_log(path, &entries).unwrap();
    let content = std::fs::read_to_string(path).unwrap();
    std::fs::remove_file(path).unwrap();
    assert!(content.contains("Mai - Gel,4500"));
    assert!(content.contains("Linh - Pedicure,3500"));
}

#[test]
fn empty_entries() {
    let path = "test_daily_empty_c48.txt";
    save_daily_log(path, &[]).unwrap();
    let content = std::fs::read_to_string(path).unwrap();
    std::fs::remove_file(path).unwrap();
    assert_eq!(content, "");
}
