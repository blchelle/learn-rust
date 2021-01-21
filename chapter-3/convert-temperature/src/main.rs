use std::io;
use std::process::exit;

fn main() {
    // Enums would clean this code up later
    // but for now let 0 mean C to F and 1
    // mean F to C

    // Prompts for and gets a mode
    let mode = get_mode();

    // Prompts the user for a temperature
    let temp = get_temp();

    // Performs the conversion
    let result = if mode == 0 {
        celsius_to_fahrenheit(temp)
    } else {
        fahrenheit_to_celsius(temp)
    };

    // Outputs the result
    println!("Result: {}", result);
}

fn get_mode() -> u8 {
    // Prompts the user for a mode
    println!("To Convert from C to F (0)");
    println!("To Convert from F to C (1)");
    println!("Which conversion would you like: ");

    // Read in the value from the command line
    let mut mode = String::new();
    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read input");

    // Parse the value from string to int
    let mode: u8 = mode
        .trim()
        .parse()
        .expect("Invalid input, 0 or 1 are valid");

    if mode != 1 && mode != 0 {
        println!("Invalid input, 0 or 1 are valid");
        exit(-1);
    }

    mode
}

fn get_temp() -> f32 {
    println!("Enter a temperature: ");

    // Gets the temperature
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read input");

    // Casts the temperature into a number
    let temp: f32 = temp
        .trim()
        .parse()
        .expect("Invalid input, input a valid number");

    temp
}

fn celsius_to_fahrenheit(num: f32) -> f32 {
    (num * 9 as f32 / 5 as f32) + 32 as f32
}

fn fahrenheit_to_celsius(num: f32) -> f32 {
    (num - 32 as f32) * 5 as f32 / 9 as f32
}
