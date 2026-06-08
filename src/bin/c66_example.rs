use std::sync::Arc;
use std::thread;

fn main() {
    let prices = Arc::new(vec![4500u32, 6000, 3500]);
    let mut handles = Vec::new();

    for _ in 0..3 {
        let prices = Arc::clone(&prices); // each thread gets its own handle
        handles.push(thread::spawn(move || prices.iter().sum::<u32>()));
    }

    let total: u32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("Total across threads: {total}");
}
