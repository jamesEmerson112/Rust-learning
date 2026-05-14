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
