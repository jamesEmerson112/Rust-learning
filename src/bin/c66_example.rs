// Arc<T> = Atomically Reference-Counted: share read-only data across THREADS. It's Rc (c28)
// with thread-safe atomic counts — plain Rc isn't safe to send between threads. (First real
// std::thread use here.) Coming from C: a refcounted shared buffer whose count is bumped with
// atomic ops, so several threads can hold it and the last one out frees it.
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
