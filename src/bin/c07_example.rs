fn shout(message: &str) -> String {
    format!("{}!", message.to_uppercase())
}

fn main() {
    let owned = String::from("hello world");

    // Borrowing: we pass a reference, `owned` stays valid
    let loud = shout(&owned);
    println!("{loud}");

    // `owned` is still usable because we only borrowed it
    println!("Original: {owned}");

    // &str literals are borrowed by default
    println!("{}", shout("rust"));
}
