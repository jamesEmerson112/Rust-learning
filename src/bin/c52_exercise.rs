use tokio::sync::mpsc;

pub async fn collect_completions() -> Vec<String> {
    // TODO: Create an mpsc::channel with buffer 10.
    // Spawn a task that sends two messages:
    //   "Mai done" and "Linh done"
    // Receive all messages and collect into a Vec.
    let (_tx, mut _rx) = mpsc::channel::<String>(10);
    tokio::spawn(async move {
        _tx.send("Mai done".to_string()).await.unwrap();
        _tx.send("Linh done".to_string()).await.unwrap();   // Linh, not Link
    });

    let mut out = Vec::new();
    while let Some(msg) = _rx.recv().await{
        out.push(msg);
    }
    out
}

#[tokio::main]
async fn main() {
    let msgs = collect_completions().await;
    for m in &msgs {
        println!("{m}");
    }
}
