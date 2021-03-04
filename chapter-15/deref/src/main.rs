fn main() {
    normal_pointer_comparison();
    println!();

    box_comparison();
    println!();

    my_box_comparison();
    println!();

    let m = MyBox::new(String::from("Rust"));

    // This function takes a &str as its parameter so why is this compiling
    // considering that we gave it a &MyBox<String>?
    //
    // Because of deref coercion, rust can turn &MyBox<String> into just
    // &String by implicitly calling deref()
    //
    // deref() on String returns a &str so Rust also implicitly calls deref
    // on the &String and converts it to a &str.
    hello_deref_coercion(&m);
    println!();

    // This is how we would have to write it if we didn't have deref coercion
    hello_deref_coercion(&(*m)[..]);
    println!();
}

fn normal_pointer_comparison() {
    println!("===== Normal Reference Comparison =====");

    let x = 5;
    let y = &x;

    // This is how regular pointers are dereferenced
    println!("{:?}", assert_eq!(x, *y));
}

fn box_comparison() {
    println!("===== Box Comparison =====");

    let x = 5;
    let y = Box::new(x);
    // The box type is dereferenced just like a regular pointer
    println!("{:?}", assert_eq!(x, *y));
}

// This is a tuple struct with only one element
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    // This is an associative type, which will be covered in chapter 19
    type Target = T;

    // This function returns a reference to the struct value
    fn deref(&self) -> &T {
        &self.0
    }
}

fn my_box_comparison() {
    println!("===== MyBox Comparison =====");
    let x = 5;
    let y = MyBox::new(x);
    // The box type is dereferenced just like a regular pointer
    println!("{:?}", assert_eq!(x, *y));
    // Under the hood, rust is actually running this
    println!("{:?}", assert_eq!(x, *(y.deref())));
}

fn hello_deref_coercion(name: &str) {
    println!("===== Hello Deref Coercion =====");
    println!("Hello, {}!", name);
}
