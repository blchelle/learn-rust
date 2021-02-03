fn main() {
    // Creating a vector with no initial values
    // Note that we have to add a type annotation here because it cannot infer
    // what we want to store inside
    let _v: Vec<i32> = Vec::new();

    // Adding elements to a vector
    // Note that this actually fails because v is immutable
    // _v.push(1);

    // Creating a vector with initial values and the vec! macro
    // We don't need to add a type annotation here because rust can infer the
    // type from the initial values.
    let _v2 = vec![1, 2, 3];

    // Adding elements to a mutable vector
    let mut _v3: Vec<i32> = Vec::new();
    _v3.push(1);

    // Just a note: once a vector goes out of scope, all of it's elements
    // are dropped as well

    // Reading from a vector
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // Reading outside the bounds of a vector
    //
    // This code panics here if uncommented
    // let _does_not_exist = &v[100];
    // let _does_not_exist = v.get(100);

    // Trying to push onto the vector while holding a reference
    // The code below fails to compile because we attempt to hold both
    // a mutable and immutable reference to v at the same time.
    //
    // This code fails to compile if uncommented
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);

    // Iterating over the values in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // Iterating over the values in a vector with mutable references
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // Using an enum to store multiple types in a vector
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!(
        "Size of SpreadsheetCell: {}",
        std::mem::size_of::<SpreadsheetCell>()
    );

    for i in &row {
        println!("{:?}", i);
    }
}
