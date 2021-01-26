// This syntax below indicates that we would like to be able to
// pretty print the value of any instances of rectangle while
// developing
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Program before structs
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} squared pixels.",
        area(width1, height1)
    );

    // Program with a tuple
    let rect_tuple = (30, 50);
    println!(
        "The area of the rectangle is {} squared pixels.",
        tuple_area(rect_tuple)
    );
    // Program with a struct
    let rect_struct = Rectangle {
        width: 30,
        height: 50,
    };

    let rect_struct_area = struct_area(&rect_struct);
    println!(
        "The area of the rectangle {:?} is {} squared pixels.",
        rect_struct, rect_struct_area
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn struct_area(dimensions: &Rectangle) -> u32 {
    dimensions.height * dimensions.width
}
