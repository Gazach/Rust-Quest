use std::io::{self, Write};
use std::process::Command;
//condition = true;
fn main() {

    loop {
        cleanwindow();

        println!("->Welcome Temperature Converter!");
        println!("==================================");
        println!("Select what kind coverter you want to do by press between 1-3");
        println!("1. Celcius to Fahrenheit/Kelvin.");
        println!("2. Fahrenheit to Celcius/Kelvin.");
        println!("3. Kelvin to Fahrenheit/Celcius.");
        println!("->Please select the menu: ");
        print!("->");
        io::stdout().flush().unwrap();
        let mut menuinput = String::new(); //input nilai untuk menu
        io::stdin().read_line(&mut menuinput).expect("Failed to read line");
        let newinput: i64 = menuinput.trim().parse().expect("Please enter a valid number");

        if newinput == 1 {
            celciusmenu();

            println!("\nWanna try other converter? [Press E] or Exit? [Press Q]");
            print!("->");
            io::stdout().flush().unwrap();
            let mut retry = String::new();
            io::stdin().read_line(&mut retry).expect("Failed to read line");
            let retry = retry.trim();
            
            // exiting program
            if retry.is_empty() || retry.to_lowercase() !=  "e" {
                break;
            }
        } else if newinput == 2 {
            fahrenheit();

            println!("\nWanna try other converter? [Press E] or Exit? [Press Q]");
            print!("->");
            io::stdout().flush().unwrap();
            let mut retry = String::new();
            io::stdin().read_line(&mut retry).expect("Failed to read line");
            let retry = retry.trim();
            
            // exiting program
            if retry.is_empty() || retry.to_lowercase() !=  "e" {
                break;
            }
        } else if newinput == 3 {
            kelvin();

            println!("\nWanna try other converter? [Press E] or Exit? [Press Q]");
            print!("->");
            io::stdout().flush().unwrap();
            let mut retry = String::new();
            io::stdin().read_line(&mut retry).expect("Failed to read line");
            let retry = retry.trim();
            
            // exiting program
            if retry.is_empty() || retry.to_lowercase() !=  "e" {
                break;
        } else {
            break;
        }
    }

    }
}
// clearing terminal window
fn cleanwindow() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}
// celcius
fn celciusmenu() {
    cleanwindow();

        println!("\nPlease input degrees Celsius that you want to convert :");
        print!("->");
        io::stdout().flush().unwrap();
    let mut celcius = String::new(); // input celcius
        io::stdin().read_line(&mut celcius).expect("Failed to read line");
            let newcelcius: f64 = celcius.trim().parse().expect("Please enter a valid number"); // change string to float
    let c = newcelcius; 
    // Celcius to Fahrenheit formula
    
    let mut f = c / 5.0;
        f = f * 9.0;
        f = f + 32.0;

    // Celcius to Kelvin formula

    let k = c + 273.15;
        let formatedk = format!("{:.2}", k); // change the format to show only last 2 digit on the coma
    
    // result converting
    println!("\nResult:");
    println!("->Fahrenheit : {} °F", f);
    println!("->Kelvin : {} K", formatedk);
}
// Fahrenheit
fn fahrenheit() {
    cleanwindow();

    println!("\nPlease input degrees fahrenheit that you want to convert :");
    print!("->");
    io::stdout().flush().unwrap();
    let mut fahrenheit = String::new(); // input celcius
        io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");
            let newfahrenheit: f64 = fahrenheit.trim().parse().expect("Please enter a valid number"); // change string to float
    let f = newfahrenheit; 
    // Fahrenheit to Celcius formula
    let mut c: f64 = f - 32.0;
    c = c / 9.0;
    c = c * 5.0;

    // Fahrenheit to Kelvin formula

    let k = c + 273.15;
    let formatedk = format!("{:.2}", k);
    //result converting
    println!("Result:");
    println!("->Celcius : {} °C", c);
    println!("->Kelvin : {} K", formatedk);
}
// kelvin
fn kelvin() {
    cleanwindow();

    println!("\nPlease input degrees kelvin that you want to convert :");
    print!("->");
    io::stdout().flush().unwrap();
    let mut kelvin = String::new(); // input celcius
        io::stdin().read_line(&mut kelvin).expect("Failed to read line");
            let newkelvin: f64 = kelvin.trim().parse().expect("Please enter a valid number"); // change string to float
    let k = newkelvin;
    // celcius
    let c = k - 273.15;

    // Fahrenheit

    let mut f = c /5.0;
    f = f * 9.0;
    f = f + 32.0;
    println!("Result:");
    println!("->Celcius : {} °C", c);
    println!("->Fahrenheit : {} °F", f);
}