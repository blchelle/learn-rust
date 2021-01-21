fn main() {
    // Scalar Types
    // Numbers
    // Addition
    let sum = 5 + 10;

    // Subtraction
    let difference = 95.5 - 4.3;

    // Multiplication
    let product = 4 * 30;

    // Division
    let quotient = 56.7 / 32.2;

    // Remainder
    let remainder = 43 % 5;

    // Prints the values
    println!("The value of sum is: {}", sum);
    println!("The value of difference is: {}", difference);
    println!("The value of product is: {}", product);
    println!("The value of quotient is: {}", quotient);
    println!("The value of remainder is: {}", remainder);
    // Characters
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    // Compound Types
    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Tuple Destructuring
    let (i, j, k) = tup;
    println!("The value of i is: {}", i);
    println!("The value of j is: {}", j);
    println!("The value of k is: {}", k);

    // Getting an index from a tuple
    let second = tup.1;
    println!("The value of second is: {}", second);

    // Arrays
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // Declaring an array with elements of type i32 with size 5
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Pulling elements from ann array
    let seventh_month = months[6];
    let first_number = a[0];

    println!("The value of seventhMonth is: {}", seventh_month);
    println!("The value of firstNumber is: {}", first_number);
}
