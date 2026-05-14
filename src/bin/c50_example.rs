async fn check_availability(slot: &str) -> bool {
    slot != "10:00"
}

#[tokio::main]
async fn main() {
    let available = check_availability("10:00").await;
    println!("10:00 available? {available}");

    let available = check_availability("11:00").await;
    println!("11:00 available? {available}");
}
