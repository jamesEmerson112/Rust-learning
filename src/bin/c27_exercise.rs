pub fn swap<T>(a: T, b: T) -> (T, T) {
    // TODO: Return (b, a) — a generic swap that works for any T.
    (b, a)
}

fn main() {
    println!("{:?}", swap(1, 2));
    println!("{:?}", swap("hi", "bye"));
}
