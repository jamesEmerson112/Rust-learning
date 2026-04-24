fn debug_string(items: &[i32]) -> String {
    format!("{:?}", items)
}

fn main() {
    let nums = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", debug_string(&nums));
}
