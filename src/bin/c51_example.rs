// async/await = cooperative concurrency. tokio::spawn launches a task that the tokio
// runtime interleaves on a small thread pool — thousands of tasks, no thread-per-task cost.
// Coming from ThreadX: like tx_thread_create, but a task is a LAZY future that only runs
// when driven (.await), and it yields voluntarily at each .await instead of being preempted.
//
// RUST GENERAL HOSPITAL: the front desk books two patients at once — neither booking
// waits for the other to finish.
async fn book(patient: &str, slot: &str) -> String {
    format!("{patient} booked at {slot}")
}

#[tokio::main]
async fn main() {
    let t1 = tokio::spawn(async { book("Mr. Hung", "10:00").await });
    let t2 = tokio::spawn(async { book("Mrs. Lan", "10:30").await });

    let r1 = t1.await.unwrap();
    let r2 = t2.await.unwrap();

    println!("{r1}");
    println!("{r2}");
}
