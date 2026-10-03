// Arc<T> stands for atomically reference-counted. It lets several threads share read-only
// data. It works like Rc from c31, but it updates its count with atomic operations, which
// stay correct when several threads run them at once. Plain Rc is not safe to send between
// threads.
// Coming from C: in C or ThreadX, this is a reference-counted shared buffer whose count is
// changed with atomic operations. Several threads can hold the buffer, and the last thread
// to release it frees it.
//
// RUST GENERAL HOSPITAL: three nurses share one supply list. Mai, Linh, and Trang each get an
// Arc handle and check the count on their own. Nobody copies the list, and it cannot be freed
// while any nurse still holds it.
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
