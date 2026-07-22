// BUG: Three technicians clock out at the end of the night — Mai, Linh, and Trang each send a
// "done" message to the front desk — but the log only ever shows Mai. The other two messages
// were delivered to the channel just fine; the front desk just stops listening after the first
// one. The code compiles and runs (no hang). Drain them all. Find and fix it.
// (This drills c50-c52: tokio mpsc channels. The tests in tests/c80_tests.rs must go green.)
use tokio::sync::mpsc;

pub async fn collect_done() -> Vec<String> {
    let (tx, mut rx) = mpsc::channel::<String>(10);

    tokio::spawn(async move {
        for tech in ["Mai", "Linh", "Trang"] {
            tx.send(format!("{tech} done")).await.unwrap();
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
    println!("front desk logged {} clock-outs:", done.len());
    for msg in &done {
        println!("  {msg}");
    }
}
