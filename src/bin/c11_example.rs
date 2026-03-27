enum Command {
    Add(i32, i32),
    Sub(i32, i32),
    Quit,
}

fn run_str_command(cmd: &str, arg: i32) {
    match (cmd, arg) {
        ("print", _) => println!("Hello!"),
        ("1", 1) | ("2", 2) | ("3", 3) => println!("A number between 1 and 3"),
        (x, 1) if x.starts_with('A') => println!("Something that starts with 'A'"),
        _ => {}
    }
}

fn run_enum_command(cmd: Command) -> String {
    match cmd {
        Command::Add(a, b) => format!("Result: {}", a + b),
        Command::Sub(a, b) => format!("Result: {}", a - b),
        Command::Quit => "Goodbye!".to_string(),
    }
}

fn main() {
    run_str_command("print", 0);
    run_str_command("1", 1);
    run_str_command("Alpha", 1);
    println!("{}", run_enum_command(Command::Add(10, 5)));
    println!("{}", run_enum_command(Command::Sub(12, 7)));
    println!("{}", run_enum_command(Command::Quit));
}
