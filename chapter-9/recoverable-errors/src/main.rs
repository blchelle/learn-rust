use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    // These are commented out because they will all panic
    // open_nonexistent_file();
    // unwrap();
    // expect();

    nested_match_error();

    let result = propagate_result();
    match result {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    };
}

fn open_nonexistent_file() {
    let f = File::open("hello2.txt");

    match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn nested_match_error() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Unable to create file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

/**
 * .unwrap() will either 'unwrap' the value inside a valid Result (Ok) or
 * it will panic with a default error message if the Result is an Err
 */
fn unwrap() {
    let f = File::open("hello3.txt").unwrap();
    println!("{:?}", f);
}

/**
 * .expect() will either 'unwrap' the value inside a valid Result (Ok) or
 * it will panic with a specified error message if the Result is an Err
 */
fn expect() {
    let f = File::open("hello3.txt").expect("Failed to open hello3.txt");
    println!("{:?}", f);
}

/**
 * This is a really long winded way of propagating errors from a called
 * function to a calling function, though it does work. The next function
 * will show a convenient shorthand to accomplish the same task.
 */
fn propagate_result() -> Result<String, io::Error> {
    // Attempts to open hello.txt
    let f = File::open("hello.txt");

    // If opening fails, then propagate the error back to main
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // Reads from f and stores the input in mutable reference s
    // Returns either the Ok(S) or Err(e) to main
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

/**
* This is the shorthand way to propagate results from a called function to
* a calling function. It makes use of the '?' operator.
*
* let mut f = File::open("hello.txt")?;
* Is equivalent to
* let mut f = match f {
*     Ok(file) => file,
*     Err(e) => return Err(e),
* };
*
* The only difference between the longhand and shorthand syntax is that the
* '?' operator in the shorthand syntax implicitly calls the 'from()' method
* which will cast the error into the type of error in the return type
*
* There is one large caveat to the ? operator. It call only be used in
* functions that return the type Result<T, E>
*/
fn propagate_shortcut() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

/**
 * There is even a shorter way to propagate a result from the called function
 * to the calling function. This way uses a technique called optional chaining
 * (at least in other languages). I'm not sure if rust has a name for it, but
 * the concept is very similar
 */
fn propagate_ultra_shortcut() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
