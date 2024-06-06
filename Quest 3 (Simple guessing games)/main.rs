// Important Note : add this to Cargo.toml before use the code!
// [dependencies]
// rand = "0.8.5"

use rand::Rng;
use std::io::{self, Write};
use std::process::Command;
fn main() {
    let mut rng = rand::thread_rng();

    loop {
        cleanwindow();
        
        print!("Random number guessing game!\n");
        // random number generator
        let rand_numbs = rng.gen_range(1..=10);

        println!("Choose between 1-10!");
        let mut answer = String::new(); 
        print!("->");
        // change string to interger
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        let newanswer: i64 = answer.trim().parse().expect("Please enter a valid number");

        //game logic
        // 1. posibilty : if user answer same as the random number = user won
        if newanswer == rand_numbs {
            println!("Congratulation! Your Correct! the answer is {}", rand_numbs);

            
            println!("\n[Press E to retry!]");
            println!("[Press Enter to exit!]");
            let mut retry = String::new();
            io::stdin().read_line(&mut retry).expect("Failed to read line");
            let retry = retry.trim();
    
            if retry.is_empty() || retry.to_lowercase() !=  "e" {
                break;
            }
        } else if newanswer > 10 || newanswer <= 0{ //2. posibilty : if user answer is greater than 10 and lower than 1, user force to restart the game or exit because of the invalid input
            println!("Please input valid answer!");

            println!("\n[Press E to retry!]");
            println!("[Press Enter to exit!]");
            let mut retry = String::new();
            io::stdin().read_line(&mut retry).expect("Failed to read line");
            let retry = retry.trim();
    
            if retry.is_empty() || retry.to_lowercase() !=  "e" {
                break;
            }
        } else { //3. posibilty : if user give the wrong answer, user will lose
            println!("Sorry Your wrong, the answer is {}", rand_numbs);

            println!("\n[Press E to retry!]");
            println!("[Press Enter to exit!]");
            let mut retry = String::new();
            io::stdin().read_line(&mut retry).expect("Failed to read line");
            let retry = retry.trim();
    
            if retry.is_empty() || retry.to_lowercase() !=  "e" {
                break;
            }
        }
    }
}
// clean recent game window
fn cleanwindow() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}
