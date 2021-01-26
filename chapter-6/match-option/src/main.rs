fn main() {
    println!("Some 5 + 1 is {}", plus_one(Some(5)));
    println!("None is {}", plus_one(None));
}

fn plus_one(x: Option<i32>) -> i32 {
    match x {
        None => 0,
        Some(i) => i + 1,
    }
}
