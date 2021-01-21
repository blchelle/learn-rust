fn main() {
    // let vs let mut
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    const Y: u32 = 5;
    println!("The value of y is: {}", Y);

    // Shadowing
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is: {}", z);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}
