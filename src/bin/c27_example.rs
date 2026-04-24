fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

fn main() {
    let (a, b) = swap(1, 2);
    println!("swap(1, 2) = ({a}, {b})");

    let (x, y) = swap("hello", "world");
    println!("swap strs = ({x}, {y})");
}
