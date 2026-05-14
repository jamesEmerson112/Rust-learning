#[path = "../src/bin/c52_exercise.rs"]
#[allow(dead_code)]
mod c52_exercise;

use c52_exercise::collect_completions;

#[tokio::test]
async fn receives_both_messages() {
    let msgs = collect_completions().await;
    assert_eq!(msgs, vec!["Mai done", "Linh done"]);
}
