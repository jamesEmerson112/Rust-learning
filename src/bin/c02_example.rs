fn build_greeting(name: &str, age: u8) -> String {
    format!("Hello, {name}! You are {age} years old.")
}

fn main() {
    let name: &str = "Ava";
    let owned: String = String::from("Ava");
    println!("&str: {name}, String: {owned}");

    let mut age: u8 = 24;
    age += 1;

    let greeting = build_greeting(name, age);
    println!("{greeting}");

    println!("Formatted: {}", format!("{} is {} years old", name, age));
}
