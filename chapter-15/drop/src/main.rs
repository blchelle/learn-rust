fn main() {
    show_off_drop();
    println!();

    manual_dropping();
    println!();
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // This function will be called implicitly by the rust compiler whenever
    // a variable of type CustomSmartPointer is freed
    fn drop(&mut self) {
        println!("Dropping the smart pointer with value {}", self.data)
    }
}

fn show_off_drop() {
    println!("===== Show Off Drop =====");
    let smart_pointer = CustomSmartPointer {
        data: String::from("Brock Chelle"),
    };

    let smart_pointer2 = CustomSmartPointer {
        data: String::from("Rust"),
    };

    println!("The value of smart_pointer is {}", smart_pointer.data);
    println!("The value of smart_pointer2 is {}", smart_pointer2.data);

    // The last memory allocated will be the first dropped
    println!("I wonder which one will get dropped first");
}

fn manual_dropping() {
    println!("===== Manual Dropping =====");

    let smart_pointer = CustomSmartPointer {
        data: String::from("Brock Chelle"),
    };

    println!("Smart Pointer Created");

    // It is ILLEGAL to manually call the drop function,
    // let the compiler do it for you
    // smart_pointer.drop();

    // You can however do this
    drop(smart_pointer);

    // Now that the smart pointer has been dropped, we can't use it again
    // This won't compile
    // println!("Smart Pointer {}, dropped", smart_pointer.data);
}
