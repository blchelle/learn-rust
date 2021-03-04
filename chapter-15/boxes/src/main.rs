fn main() {
    new_box();
    println!();

    use_recursive_list();
    println!();
}

fn new_box() {
    println!("===== New Box =====");

    // The value is stored on the heap
    // b is a pointer to the memory in the heap where the data is kept
    let b = Box::new(5);
    println!("b = {}", b)

    // Both the pointer b and the data on the heap are deallocated after
    // the end of this function
}

// Recursive Data Types
// This is invalid because List contains a list which could contain another
// list and so on. Which means it could have infinite memory
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// This however is a perfect use case for boxes
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn use_recursive_list() {
    println!("===== Recursive List =====");

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("Recursive List: {:?}", list);
}
