#[path = "../src/bin/c80_exercise.rs"]
#[allow(dead_code)]
mod c80_exercise;

use c80_exercise::*;

#[tokio::test]
async fn collects_every_clock_out() {
    let done = collect_done().await;
    assert_eq!(done, vec!["Mai done", "Linh done", "Trang done"]);
}
