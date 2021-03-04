fn main() {
    reference_counting();
    println!();
}

use std::rc::Rc;

// This implementation would not work because we need to maintain a reference
// counter so that the rust compiler knows when to deallocate the memory
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn reference_counting() {
    println!("===== Reference Counting =====");

    let a = Rc::new(Cons(5, Rc::new(Cons(3, Rc::new(Nil)))));
    println!("Count after creating a {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b {}", Rc::strong_count(&a));

    let c = Cons(4, Rc::clone(&a));
    println!("Count after creating c {}", Rc::strong_count(&a));

    drop(b);
    println!("Count after dropping b {}", Rc::strong_count(&a));

    drop(c);
    println!("Count after dropping c {}", Rc::strong_count(&a));
}
