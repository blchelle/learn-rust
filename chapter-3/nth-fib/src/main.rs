use std::io;
use std::process::exit;

fn main() {
    let n = get_n();
    get_nth_fib(n);
}

fn get_n() -> u32 {
    // Prompts for n
    println!("Please input a positive integer");

    // Gets the input n and converts it to an integer
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input");
    let n: u32 = n
        .trim()
        .parse()
        .expect("Invalid input, n must be a positive integer");

    // Returns n
    n
}

fn get_nth_fib(n: u32) {
    let mut prev: u128 = 0;
    let mut current: u128 = 1;

    // Base cases for the fibonacci sequence
    if n == 0 {
        println!("Invalid input 0");
        exit(-1);
    } else if n == 1 {
        current = 0;
    } else if n == 2 {
        current = 1;
    } else {
        // For loop with an unused counter
        for _ in 3..n + 1 {
            let next = prev + current;
            prev = current;
            current = next;
        }
    }

    println!(
        "The value of the {}th number in the fibonacci sequence is {}",
        n, current
    );
}
