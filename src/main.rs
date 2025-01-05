use std::io::{self, Write};

fn main() {
    println!("Hello, world!");
    println!("Welcome to Connor Denihan's Rust Calculator!");

    let number_one = read_input("Please enter the first number: ");
    let number_two = read_input("Please enter the second number: ");
    let operation = read_operation();

    match operation {
        1 => println!("{} + {} = {}", number_one, number_two, number_one + number_two),
        2 => println!("{} - {} = {}", number_one, number_two, number_one - number_two),
        3 => println!("{} * {} = {}", number_one, number_two, number_one * number_two),
        4 => {
            if number_two == 0.0 {
                println!("Error: Division by zero is not allowed.");
            } else {
                println!("{} / {} = {}", number_one, number_two, number_one / number_two);
            }
        }
        _ => println!("Invalid operation selected!"),
    }
}

fn read_input(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn read_operation() -> u32 {
    loop {
        println!("Please enter the operation you would like to perform: ");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(num) if num >= 1 && num <= 4 => return num,
            _ => println!("Invalid input. Please enter a number between 1 and 4."),
        }
    }
}