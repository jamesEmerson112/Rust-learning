pub fn largest<T: Ord + Copy>(items: &[T]) -> Option<T> {
    // TODO: Return the largest item, or None when the slice is empty.
    let _ = items;
    None
}

fn main() {
    let values = [4, 1, 8, 3];
    println!("Largest: {:?}", largest(&values));
}
