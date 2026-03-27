use std::fmt::format;

pub enum Command {
    Add(i32, i32),
    Sub(i32, i32),
    Quit,
}

pub fn run_command(cmd: Command) -> String {
    // TODO: Match on Command and return:
    // Add(a, b) => "Result: <sum>"
    // Sub(a, b) => "Result: <difference>"
    // Quit => "Goodbye!"
    match cmd {
        Command::Add(num1, num2)  => format!("{}", num1 + num2),
        Command::Sub(num1, num2)  => format!("{}", num1 - num2) ,
        Command::Quit                       => "Goodbye!".to_string()
    }
}

fn main() {
    println!("{}", run_command(Command::Add(2, 3)));
    println!("{}", run_command(Command::Sub(9, 4)));
    println!("{}", run_command(Command::Quit));
}
