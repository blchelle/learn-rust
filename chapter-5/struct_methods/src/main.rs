#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Syntax for defining methods on a struct
// Seems like theres a lot of parallels between structs and
// classes in other languages
//
// Another note is that we can technically have multiple impl blocks
// for a single class. Though, the benefits of doing so aren't clear yet
impl Rectangle {
    // First parameter is a borrowed version of the instance aka self
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn gen_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };

    println!("The area of rect1 is {} square pixels", rect1.area());

    let rect2 = Rectangle {
        width: 40,
        height: 20,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 40,
    };

    println!("Rect1 can hold Rect2: {}", rect1.can_hold(&rect2));
    println!("Rect1 can hold Rect3: {}", rect1.can_hold(&rect3));
    println!("Rect3 can hold Rect1: {}", rect3.can_hold(&rect1));

    let square = Rectangle::gen_square(7);
    println!("The area of square is {} square pixels", square.area());
    println!("Rect1 can hold Square: {}", rect1.can_hold(&square));
}
