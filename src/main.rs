// Import the standard library's input/output module
use std::io;

// Import the `user_utils` and `math_utils` modules for helper functions
mod user_utils;
mod math_utils;

// The main function of the program
fn main() {
    // Loop indefinitely until the user chooses to quit
    loop {
        // Print the menu of options
        println!();
        println!("What would you like to do?");
        println!("1. Calculate Area");
        println!("2. Calculate Volume");
        println!("3. Even or Odd");
        println!("4. Reverse a String");
        println!("5. Number of vowels and consonants");
        println!("6. Multiplication Table");
        println!("7. Quit");

        // Read the user's input as a string
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // Parse the user's input as an unsigned 32-bit integer
        let choice: u32 = choice
            .trim()
            .parse()
            .expect("Please type a number!");

        // Match the user's choice with the appropriate menu option
        match choice {
            1 => calculate_area(),
            2 => calculate_volume(),
            3 => even_or_odd(),
            4 => reverse_string(),
            5 => count_vowels_and_consonants(),
            6 => multiplication_table(),
            7 => break,
            // If the user inputs an invalid choice, print an error message
            _ => println!("Please choose a valid option."),
        }
    }
}

// Calculate the volume of a rectangle based on its width, height, and depth
fn calculate_volume() {
    let width = user_utils::read_input_number("Width: ");
    let height = user_utils::read_input_number("Height: ");
    let depth = user_utils::read_input_number("Depth: ");

    println!(
        "The volume of the rectangle is {} cubic units.",
        math_utils::volume(width, height, depth)
    );
}

// Calculate the area of a rectangle based on its width and height
fn calculate_area() {
    let width = user_utils::read_input_number("Width: ");
    let height = user_utils::read_input_number("Height: ");

    println!(
        "The area of the rectangle is {} square units.",
        math_utils::area(width, height)
    );
}

// Determine whether a number is even or odd
fn even_or_odd() {
    let number = user_utils::read_input_number("Number: ");

    if number % 2 == 0 {
        println!("{} is even.", number);
    } else {
        println!("{} is odd.", number);
    }
}

// Reverse a string input by the user
fn reverse_string() {
    let input = user_utils::read_input_string("String: ");

    let mut reversed = String::new();
    for c in input.chars().rev() {
        reversed.push(c);
    }

    println!("Reversed: {}", reversed);
}

// Count the number of vowels and consonants in a string
fn count_vowels_and_consonants() {
    let input = user_utils::read_input_string("String: ");

    let mut vowels = 0;
    let mut consonants = 0;
    for c in input.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
            'A' | 'E' | 'I' | 'O' | 'U' => vowels += 1,
            ' ' => (),
            _ => consonants += 1,
        }
    }

    println!("Vowels: {}", vowels);
    println!("Consonants: {}", consonants);
}

// Print the multiplication table for a number
fn multiplication_table() {
    let number = user_utils::read_input_number("Number: ");
    let steps = user_utils::read_input_number("Steps: ");

    for i in 1..=steps {
        println!("{} x {} = {}", number, i, number * i);
    }
}