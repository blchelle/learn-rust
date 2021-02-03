mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Bring hosting into scope so we don't have to specify the entire path
use crate::front_of_house::hosting;

// Brings hosting into scope with a relative path
// This won't compile because it is a duplicate import
// use self::front_of_house::hosting;

// You could bring 'add_to_waitlist' into scope as well,
// but this is not conventional
// use self::front_of_house::hosting::add_to_waitlist;

// However, it is conventional to bring structs enums directly into scope like
// use std::collections::HashMap;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn main() {}

// We need two types named Result, but they're from different crates
// In this case we just import their parent for clarity
use std::fmt;
use std::io;

// Doesn't compile since we don't actually return the type
fn function1() -> fmt::Result {
    // --snip--
}

// Doesn't compile since we don't actually return the type
fn function2() -> io::Result<()> {
    // --snip--
}

// We can rename the result type instead of importing the parents
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

// Doesn't compile since we don't actually return the type
fn function3() -> FmtResult {
    // --snip--
}

// Doesn't compile since we don't actually return the type
fn function4() -> IoResult {
    // --snip--
}

// Here's a handy shorthand for importing multiple items from the same crate
// Original:
// use std::cmp::Ordering;
// use std::io;
//
// Shorthand
// use::{cmp::Ordering, io};

// Glob operator
// Brings all items from a path into scope
// use std::collections::*;
