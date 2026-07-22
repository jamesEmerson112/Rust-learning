// Bug Hunt drill: drain the whole channel. `while let Some(msg) = rx.recv().await` loops until
// every sender is dropped and the queue is empty. Use a single `if let` and you read exactly
// ONE message and quit, stranding the rest — even though they were delivered fine.
// Coming from ThreadX: recv() is tx_queue_receive. You loop it until the queue is closed;
// calling it once and walking away leaves messages sitting in the queue unread.
use tokio::sync::mpsc;

pub async fn collect_done() -> Vec<String> {
    let (tx, mut rx) = mpsc::channel::<String>(10);

    // One task sends all three sequentially, so the order is deterministic: Mai, Linh, Trang.
    tokio::spawn(async move {
        for tech in ["Mai", "Linh", "Trang"] {
            tx.send(format!("{tech} done")).await.unwrap();
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
    println!("front desk logged {} clock-outs:", done.len());
    for msg in &done {
        println!("  {msg}");
    }
}
