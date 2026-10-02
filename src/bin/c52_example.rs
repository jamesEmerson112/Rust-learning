// Tasks don't share memory — they pass messages over a channel. mpsc = Multi-Producer,
// Single-Consumer: many senders (tx), one receiver (rx). recv().await waits without blocking.
// Coming from ThreadX: this is tx_queue_send / tx_queue_receive — a message queue between
// tasks — but the compiler stops you from touching data you've already handed off.
//
// RUST GENERAL HOSPITAL: nurses report to the nurses' station as each round finishes.
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(10);

    tokio::spawn(async move {
        tx.send("Mai finished rounds on Ward 3".to_string()).await.unwrap();
        tx.send("Linh finished rounds on Ward 5".to_string()).await.unwrap();
    });

    while let Some(msg) = rx.recv().await {
        println!("Nurses' station: {msg}");
    }
}
