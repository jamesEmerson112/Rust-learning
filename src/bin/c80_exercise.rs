// BUG: Three nurses clock out at the end of the night shift. Mai, Linh, and Trang each send a
// "done" message to the nurses' station, but the log only ever shows Mai. The other two
// messages were delivered to the channel without any problem. The station stops listening
// after the first one. The code compiles and runs, and it does not hang. Find the bug and
// fix it.
// This drills tokio mpsc channels, which have many senders and one receiver, from c50 to
// c52. The tests in tests/c80_tests.rs must pass.
use tokio::sync::mpsc;

pub async fn collect_done() -> Vec<String> {
    let (tx, mut rx) = mpsc::channel::<String>(10);

    tokio::spawn(async move {
        for nurse in ["Mai", "Linh", "Trang"] {
            tx.send(format!("{nurse} done")).await.unwrap();
        }
    });

    let mut out = Vec::new();
    if let Some(msg) = rx.recv().await {
        out.push(msg);
    }
    out
}

#[tokio::main]
async fn main() {
    let done = collect_done().await;
    println!("nurses' station logged {} clock-outs:", done.len());
    for msg in &done {
        println!("  {msg}");
    }
}
