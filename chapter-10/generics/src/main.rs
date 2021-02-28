// This is a generic type where both x, y must be of the same type
struct Point<T> {
    x: T,
    y: T,
}

// This is a generic type where x, y can be of different types
struct Point2<T, U> {
    x: T,
    y: U,
}

// We can also use our generics in method definitions, though what I don't
// understand fully here is why we have to add the parameter definition
// after both 'impl' and 'Point'
//
// After looking a little further, <T> has to come after impl because otherwise
// the compiler would not have any way of knowing that the T in Point<T> is a
// generic and not a concrete type.
//
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Unlike the previous impl, this impl is only on the concrete type Point<f32>.
// So a Point<i32> would not have access to these methods. The same goes for
// any types T that are not f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        let sos = self.x.powi(2) + self.y.powi(2);
        sos.sqrt()
    }
}

// This impl shows us that we are not restricted to the originally defined
// type parameters. Here, we introduce two new generic params V, W.
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

/**
 * This simply finds the largest integer in a list
 * This will be converted into a generic below
 */
fn largest_i32(list: &[i32]) -> i32 {
    println!("===== Largest Int =====");

    // Initializes largest to be -inf
    let mut largest = list[0];

    // Iterates through the slice, reassigning the largest every time
    // a larger number is found
    for item in list {
        if *item > largest {
            largest = *item;
        }
    }

    // Returns the largest
    largest
}

/**
 * This simply finds the largest char in a list (in terms of unicode value)
 * This will be converted into a generic below
 */
fn largest_char(list: &[char]) -> char {
    println!("===== Largest Char =====");

    // Initializes largest to be -inf
    let mut largest = list[0];

    // Iterates through the slice, reassigning the largest every time
    // a larger number is found
    for item in list {
        if *item > largest {
            largest = *item;
        }
    }

    // Returns the largest
    largest
}

/**
 * This finds the largest value in a list of generic type T
 * Right now this function won't compile because of the use of the '>' operator
 * To use the '>' operator (or the '<', '==') the two values need to be
 * comparable. Not all types in rust are guaranteed to be comparable. For
 * example, a custom struct. So we need to somehow indicate that T must be a
 * type which can be ordered/compared
 */
fn largest_generic<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    println!("===== Largest Generic =====");

    // Initializes largest to be -inf
    let mut largest = list[0];

    // Iterates through the slice, reassigning the largest every time
    // a larger number is found
    for item in list {
        if *item > largest {
            largest = *item;
        }
    }

    // Returns the largest
    largest
}

fn main() {
    let int_list = vec![100, 45, 200, 1, -7];
    println!("{}", largest_i32(&int_list));
    println!();

    let char_list = vec!['c', 'a', 'B', 'z', '9'];
    println!("{}", largest_char(&char_list));
    println!();

    println!("{}", largest_generic(&int_list));
    println!("{}", largest_generic(&char_list));

    // Two uses of the generic struct we created above
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 5.0, y: 10.0 };

    println!("Integer Point: {}, {}", integer.x, integer.y);
    println!("Float Point: {}, {}", float.x, float.y);

    // But this doesn't compile because the generic type must be the same
    // let both = Point { x: 5, y: 10.0 };

    // This uses the second point struct which takes 2 generic types T, U
    // Though T, U need not be different
    let both = Point2 { x: 5, y: 10.0 };
    let integer2 = Point2 { x: 5, y: 10 };

    // This uses the mixup method, so it takes x from both and y from float.
    // Which still ends up being two integers
    let mixed = both.mixup(integer2);
    println!("Mixed Point: {}, {}", mixed.x, mixed.y);
}
