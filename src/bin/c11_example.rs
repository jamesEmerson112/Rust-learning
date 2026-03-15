enum Command {
    Add(i32, i32),
    Sub(i32, i32),
    Quit,
}

fn run_command(cmd: Command) -> String {
    match cmd {
        Command::Add(a, b) => format!("Result: {}", a + b),
        Command::Sub(a, b) => format!("Result: {}", a - b),
        Command::Quit => "Goodbye!".to_string(),
    }
}

fn main() {
    println!("{}", run_command(Command::Add(10, 5)));
    println!("{}", run_command(Command::Sub(12, 7)));
    println!("{}", run_command(Command::Quit));
}
