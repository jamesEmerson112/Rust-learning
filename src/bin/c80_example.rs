// Bug Hunt drill: read every message from the channel. The loop
// `while let Some(msg) = rx.recv().await` keeps going until every sender is dropped and the
// queue is empty. A single `if let` reads exactly one message and stops. The remaining
// messages stay in the queue unread, even though they were delivered correctly.
// Coming from C: in ThreadX, recv() is tx_queue_receive. You call it in a loop until the
// queue is closed. If you call it once and stop, the other messages sit in the queue unread.
use tokio::sync::mpsc;

pub async fn collect_done() -> Vec<String> {
    let (tx, mut rx) = mpsc::channel::<String>(10);

    // One task sends all three messages in turn, so they always arrive as Mai, Linh, Trang.
    tokio::spawn(async move {
        for nurse in ["Mai", "Linh", "Trang"] {
            tx.send(format!("{nurse} done")).await.unwrap();
        }
    });

    let mut out = Vec::new();
    while let Some(msg) = rx.recv().await {
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
