fn largest<T: Ord + Copy>(items: &[T]) -> Option<T> {
    let mut iter = items.iter().copied();
    let mut largest = iter.next()?;
    for item in iter {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

fn main() {
    let numbers = vec![3, 9, 2, 14, 5];
    let letters = vec!['a', 'z', 'm'];
    println!("Largest number: {:?}", largest(&numbers));
    println!("Largest letter: {:?}", largest(&letters));
}
