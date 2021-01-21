fn main() {
    println!("Hello, world!");
    another_function();
    function_with_params(5, 6);
    function_with_expr();
    println!("The return of five is: {}", five());
}

// Basic Declaration
fn another_function() {
    println!("Another function.");
}

// Declaration with parameter
fn function_with_params(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Function with expression
fn function_with_expr() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // Expressions don't end with semicolons
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Function with a return
fn five() -> i32 {
    5
}
