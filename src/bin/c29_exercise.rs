pub fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    // TODO: Return whichever of `a` or `b` has the longer length.
    // If they're equal, return `a`.
    // The lifetime 'a means: the output reference lives at least as long
    // as the shorter-lived of the two inputs.
    if a.len() >= b.len() {a} else {b}
}

fn main() {
    let s1 = "hello";
    let s2 = "hi";
    println!("{}", longer(s1, s2));
}
