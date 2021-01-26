fn main() {
    let some_u8_value = Some(3);

    // Before if-let
    match some_u8_value {
        Some(3) => println!("Three!"),
        _ => println!("Not Three!"),
    }

    // After if-let
    // if-let is really just syntactic sugar for match where
    // you only have 1 useful case
    if let Some(3) = some_u8_value {
        println!("Three!")
    }
}
