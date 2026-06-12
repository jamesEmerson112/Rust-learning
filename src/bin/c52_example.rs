// Tasks don't share memory — they pass messages over a channel. mpsc = Multi-Producer,
// Single-Consumer: many senders (tx), one receiver (rx). recv().await waits without blocking.
// Coming from ThreadX: this is tx_queue_send / tx_queue_receive — a message queue between
// tasks — but the compiler stops you from touching data you've already handed off.
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(10);

    tokio::spawn(async move {
        tx.send("Mai finished Gel Manicure".to_string()).await.unwrap();
        tx.send("Linh finished Pedicure".to_string()).await.unwrap();
    });

    while let Some(msg) = rx.recv().await {
        println!("Front desk: {msg}");
    }
}
