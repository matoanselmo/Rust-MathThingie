use std::io;
use std::io::Write;

pub fn read_input_number(prompt: &str) -> u32 {
    print!("{}", prompt);
    io::stdout()
        .flush()
        .expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim()
        .parse()
        .expect("Please type a number!")
}

pub fn read_input_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout()
        .flush()
        .expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}