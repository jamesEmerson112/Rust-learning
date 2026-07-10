pub async fn book(client: &str, slot: &str) -> String {
    format!("{client} booked at {slot}")
}

pub async fn book_two() -> (String, String) {
    // TODO: Spawn two tasks using tokio::spawn.
    // Task 1: book("Mai", "10:00")
    // Task 2: book("Linh", "10:30")
    // Await both and return the results as a tuple.
    // (String::new(), String::new())
    let t1 = tokio::spawn(async{book("Mai", "10:00").await});
    let t2 = tokio::spawn(async{book("Linh", "10:30").await});

    let r1 = t1.await.unwrap();
    let r2 = t2.await.unwrap();

    (r1, r2)

}

#[tokio::main]
async fn main() {
    let (a, b) = book_two().await;
    println!("{a}");
    println!("{b}");
}
