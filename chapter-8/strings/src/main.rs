// So what's the difference between the str, &str and String
// str, also known as a string slice is a string literal stored in the programs
// binary
// &str, is the borrowed form of str
// String is an owned string type
//
// For example if we're getting input at the command line we should store it as
// a String because it is not known ahead of time and cannot be stored in the
// programs binary

fn main() {
    creating_a_string();
    println!();

    slice_to_string();
    println!();

    string_utf8();
    println!();

    appending_to_string();
    println!();

    join_strings();
    println!();

    format_strings();
    println!();

    indexing_strings();
    println!();
}

fn creating_a_string() {
    println!("===== Initializing a String =====");

    // Here, I create an empty string and push the word hello into the string
    // one letter at a time. A better alternative would be to use .push_str()
    let mut s = String::new();
    s.push('h');
    s.push('e');
    s.push('l');
    s.push('l');
    s.push('o');
    println!("{}", s);
}

fn slice_to_string() {
    println!("===== Slice to String =====");
    let s = "this uses .to_string()".to_string();
    let s2 = String::from("this uses String::from()");
    println!("Method 1: {}", s);
    println!("Method 2: {}", s2);
}

/**
 * Strings in rust support all UTF-8 characters,
 * Here are some examples.
 */
fn string_utf8() {
    println!("===== UTF-8 Strings =====");

    println!("{}", String::from("السلام عليكم"));
    println!("{}", String::from("Dobrý den"));
    println!("{}", String::from("Hello"));
    println!("{}", String::from("שָׁלוֹם"));
    println!("{}", String::from("नमस्ते"));
    println!("{}", String::from("こんにちは"));
    println!("{}", String::from("안녕하세요"));
    println!("{}", String::from("你好"));
    println!("{}", String::from("Olá"));
    println!("{}", String::from("Здравствуйте"));
    println!("{}", String::from("Hola"));
}

/**
 * This function shows the .push_str() method mentioned earlier
 */
fn appending_to_string() {
    println!("===== Appending to a String =====");
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("Concatenated strings: {}", s);
}

/**
 * Joins two values of String type with the + operator. Note that in order to
 * do this we have to take ownership of the first string in the concatenation
 * The following strings in the concatenation have to be borrowed.
 *
 * This is because the + operator for strings calls add() under the hood which
 * requires ownership of the value
 */
fn join_strings() {
    println!("===== Joining Strings with '+' =====");
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = String::from(" My name is Brock");
    let s4 = s1 + &s2 + &s3;
    // This wont compile because we gave ownership of s1 to s3
    // println!("First string is: {}", s1);
    println!("Second string is: {}", s2);
    println!("Third string is: {}", s3);
    println!("Joined Strings: {}", s4);
}

/**
 * Another way to join strings is with the format!() macro. In my opinion
 * this method is much cleaner since all the original string retain
 * ownership of their values
 */
fn format_strings() {
    println!("===== Creating a string with format =====");
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = String::from(" My name is Brock");
    let s = format!("{}{}{}", s1, s2, s3);
    println!("First string is: {}", s1);
    println!("Second string is: {}", s2);
    println!("Third string is: {}", s3);
    println!("Joined Strings: {}", s);
}

/**
 * Accessing elements in strings is actually quite complex
 * We cannot use the conventional syntax string[i] in Rust because of the
 * nature of utf-8 characters.
 */
fn indexing_strings() {
    println!("===== How do I index strings? =====");

    let s = String::from("Hello");
    let s_chars = s.chars();

    for character in s_chars {
        print!("{}", character);
    }
}
