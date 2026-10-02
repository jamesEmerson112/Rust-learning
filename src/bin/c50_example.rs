async fn check_availability(slot: &str) -> bool {
    slot != "10:00"
}

#[tokio::main]
async fn main() {
    let available = check_availability("10:00").await;
    println!("Clinic slot 10:00 free? {available}");

    let available = check_availability("11:00").await;
    println!("Clinic slot 11:00 free? {available}");
}
