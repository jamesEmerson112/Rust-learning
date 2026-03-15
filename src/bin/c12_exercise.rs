pub fn safe_get(items: &[i32], index: usize) -> Option<i32> {
    // TODO: Return Some(value) if index is valid, None otherwise.
    let _ = (items, index);
    None
}

fn main() {
    let nums = [10, 20, 30];
    println!("{:?}", safe_get(&nums, 1));
    println!("{:?}", safe_get(&nums, 99));
}
