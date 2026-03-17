fn build_greeting(name: &mut str, age: &[u8], name2 : String) -> String {
    format!("Hello, {name}! You are {age} years old.")
}

fn main() {
    let mut name: &str = "Ava";
    let owned: String = String::from("Ava");
    println!("&str: {name}, String: {owned}");

    // name.push('!'); // This would cause a compile error since name is a &str
    let mut owned_mut = owned.clone();
    owned_mut.push('!');
    let mut age: u8 = 24;
    println!("{owned}");
    age += 1;

    let greeting = build_greeting(name, &age, owned_mut);
    println!("{greeting}");

    let my_pattern: &str = "Formatted: {}";

    println!(my_pattern, format!("{} is {} years old", name, age));
}
