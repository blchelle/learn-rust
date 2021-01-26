struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let brock_chelle = User {
        email: String::from("Brocklchelle@gmail.com"),
        username: String::from("blchelle"),
        active: false,
        sign_in_count: 1,
    };

    println!("My email is {}", brock_chelle.email);
    println!("My username is {}", brock_chelle.username);
    println!("Active: {}", brock_chelle.active);
    println!("I've signed in {} times\n", brock_chelle.sign_in_count);

    // Below is invalid because the data is immutable
    // brock_chelle.email = String::from("bchelle@ualberta.ca")

    let mut jerry_smith = User {
        email: String::from("jsmith@gmail.com"),
        username: String::from("jerry"),
        active: true,
        sign_in_count: 8,
    };

    // This is valid because it is declared at mutable
    jerry_smith.email = String::from("jsmith");
    println!("My email is {}", jerry_smith.email);
    println!("My username is {}", jerry_smith.username);
    println!("Active: {}", jerry_smith.active);
    println!("I've signed in {} times\n", jerry_smith.sign_in_count);

    let allison_blue = build_user(String::from("ab@gmail.com"), String::from("ablue"));
    println!("My email is {}", allison_blue.email);
    println!("My username is {}", allison_blue.username);
    println!("Active: {}", allison_blue.active);
    println!("I've signed in {} times\n", allison_blue.sign_in_count);

    // Struct update syntax. Very similar to the TS spread operator
    let bob_mills = User {
        email: String::from("bmills@gmail.com"),
        username: String::from("bmills"),
        ..allison_blue
    };
    println!("My email is {}", bob_mills.email);
    println!("My username is {}", bob_mills.username);
    println!("Active: {}", bob_mills.active);
    println!("I've signed in {} times\n", bob_mills.sign_in_count);

    // Tuple Struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let red = Color(255, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Red: {}, {}, {}", red.0, red.1, red.2);
    println!("Origin: {}, {}, {}", origin.0, origin.1, origin.2);

    // The below line is invalid
    // Even though each field has the same type
    // The structs Color and Point are different
    // let black: Color = origin;
}

// Function to build a users
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
