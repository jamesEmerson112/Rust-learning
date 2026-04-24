fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let s1 = String::from("short");
    let s2 = String::from("a much longer sentence");
    println!("longer = {:?}", longer(&s1, &s2));
}
