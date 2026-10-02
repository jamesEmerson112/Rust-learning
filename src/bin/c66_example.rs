// Arc<T> = Atomically Reference-Counted: share read-only data across THREADS. It's Rc (c31)
// with thread-safe atomic counts — plain Rc isn't safe to send between threads. Coming from
// C/ThreadX: a refcounted shared buffer whose count is bumped with atomic ops, so several
// threads can hold it and the last one out frees it.
//
// RUST GENERAL HOSPITAL: one supply list, three nurses. Mai, Linh, and Trang each get an Arc
// handle and independently double-check the count — nobody copies the list, nobody frees it early.
use std::sync::Arc;
use std::thread;

fn main() {
    let supply_list = Arc::new(vec![4000u32, 6500, 3500]); // saline on hand, in ml

    let mut nurses = Vec::new();
    for nurse in ["Mai", "Linh", "Trang"] {
        let list = Arc::clone(&supply_list); // each nurse gets a handle
        nurses.push(thread::spawn(move || {
            let count: u32 = list.iter().sum();
            println!("[{nurse}] counts {count} ml");
            count
        }));
    }

    let combined: u32 = nurses.into_iter().map(|h| h.join().unwrap()).sum();
    println!("[ward] all three counts together: {combined} ml");
}
