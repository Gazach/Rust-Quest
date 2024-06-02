use std::io::{self, Write};
use std::process;

fn main() {
    println!("Calculator project!\n");
    println!("select what calculations you want to do.");
    println!("==============================");
    println!("(1) summation [+]");
    println!("(2) subtraction [-]");
    println!("(3) multiplication [x]");
    println!("(4) division [/]");
    println!("==============================");
    println!("choose between 1-4");
    let mut input = String::new();

    // Prompt and read first number
    print!("-> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: f64 = input.trim().parse().expect("Please enter a valid number");

    if a > 4.0 {
        print!("Input false! 'the number you entered is not exist in the menu'.");
        wait_for_enter();
        process::exit(0);
    } else if a == 1.0 {
        summation();
    }
    else if a == 2.0 {
        subtraction();
    }
    else if a == 3.0 {
        multiplication();
    }
    else if a == 4.0 {
        division();
    }

        wait_for_enter();
}

fn summation() {
    println!("Summation.\n");
    let mut inputmath = String::new();
    
    print!("Input first number : ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    io::stdin().read_line(&mut inputmath).expect("Failed to read line");
    let a: f64 = inputmath.trim().parse().expect("Please type a number!");

    inputmath.clear();

    print!("Input second number :  ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    io::stdin().read_line(&mut inputmath).expect("Failed to read line");
    let b: f64 = inputmath.trim().parse().expect("Please type a number!");

    let result = a + b;

    println!("\nResult : {}", result);
}
fn subtraction() {
    println!("subtraction.\n");
    let mut inputmath = String::new();
    
    print!("Input first number : ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    io::stdin().read_line(&mut inputmath).expect("Failed to read line");
    let a: f64 = inputmath.trim().parse().expect("Please type a number!");

    inputmath.clear();

    print!("Input second number :  ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    io::stdin().read_line(&mut inputmath).expect("Failed to read line");
    let b: f64 = inputmath.trim().parse().expect("Please type a number!");

    let result = a - b;

    println!("\nResult : {}", result);
}
fn multiplication() {
    println!("multiplication.\n");
    let mut inputmath = String::new();
    
    print!("Input first number : ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    io::stdin().read_line(&mut inputmath).expect("Failed to read line");
    let a: f64 = inputmath.trim().parse().expect("Please type a number!");

    inputmath.clear();

    print!("Input second number :  ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    io::stdin().read_line(&mut inputmath).expect("Failed to read line");
    let b: f64 = inputmath.trim().parse().expect("Please type a number!");

    let result = a * b;

    println!("\nResult : {}", result);
}
fn division() {
    println!("division.\n");
    let mut inputmath = String::new();
    
    print!("Input first number : ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    io::stdin().read_line(&mut inputmath).expect("Failed to read line");
    let a: f64 = inputmath.trim().parse().expect("Please type a number!");

    inputmath.clear();

    print!("Input second number :  ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    io::stdin().read_line(&mut inputmath).expect("Failed to read line");
    let b: f64 = inputmath.trim().parse().expect("Please type a number!");

    let result = a / b;

    println!("\nResult : {}", result);

}
// wait for exit program
fn wait_for_enter() {
    println!("\nPress Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("Wait...");
}