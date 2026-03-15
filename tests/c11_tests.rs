#[path = "../src/bin/c11_exercise.rs"]
#[allow(dead_code)]
mod c11_exercise;

use c11_exercise::{Command, run_command};

#[test]
fn run_command_add() {
    assert_eq!(run_command(Command::Add(4, 5)), "Result: 9");
}

#[test]
fn run_command_sub() {
    assert_eq!(run_command(Command::Sub(10, 3)), "Result: 7");
}

#[test]
fn run_command_quit() {
    assert_eq!(run_command(Command::Quit), "Goodbye!");
}
