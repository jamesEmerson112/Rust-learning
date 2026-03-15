pub fn build_greeting(name: &str, age: u8) -> String {
    // TODO: Return exactly: "Hello, <name>! You are <age> years old."
    let _ = (name, age);
    // String::new();
    format!("Hello, {name}! You are {age} years old.")
}

fn main() {
    println!("{}", build_greeting("Ava", 25));
}
