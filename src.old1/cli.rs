use std::env;

pub fn run() {
    // collects arguments from the command line
    let args: Vec<String> = env::args().collect();

    let command = args[1].clone();
    let status = "Okay";
    let name = "Ryan";

    println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is: {}", status);
    } else {
        println!("That is not a valid command!");
    }
}