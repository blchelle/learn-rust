fn main() {
    let number = 6;

    // Note the lack of parentheses
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // The condition must be a bool, unlike javascript where
    // it will cast your number to a bool.
    // Also, curly braces are necessary even for one liners
    if number != 0 {
        println!("number is not zero")
    }

    // Multiple branches, nothing too crazy here
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Ternary operator
    //
    // This isn't really a ternary operator, it's really just the rust
    // syntax for if-else statements condensed onto one line
    //
    // If statements are expressions, meaning they can return a value
    //
    // However, for the sake of type inference the return type in the if
    // branch and the else branch must be the same
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {}", number);
}
