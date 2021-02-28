use std::fmt::Display;

/**
 * Consider the following function which has an outer scope (the function body)
 * and an inner scope (the artificial scope in the middle)
 *
 * r is declared in the outer scope, as such it will live until the function
 * returns x is declared in the inner scope, it only lives until the inner
 * scope ends.
 *
 * In the inner scope we try to get r to reference x. This is invalid because
 * once the inner scope ends, x will be cleaned up and r will point to nothing
 *
 * Luckily, the rust compiler is able to detect this and warns us
 */
fn prevent_dangling_ref() {
    println!("===== Prevent Dangling Pointer =====");
    let r;

    {
        let x = 5;
        r = &x;
        println!("r: {}", r)
    }

    // The below line will cause a compiler error because we are trying to use
    // r, which reference x, which has gone out of scope and no longer exists
    // println!("r: {}", r);
}

/**
 * Because this function can return either s1 or s2 and we don't have any way
 * of knowing the lifetimes of s1 and s2 we need to specify a generic lifetime
 * that guarantees that all parameters and returns annotated with the lifetime
 * will live at least as long as the lifetime parameter.
 */
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

/**
 * In this function, we know that we are returning s1 every time. So we can
 * remove the lifetime parameter 'a on s2 because it doesn't matter. The only
 * thing that we need to know is that the return slice will live at least as
 * long as the parameter s1.
 */
fn longest2<'a>(s1: &'a str, s2: &str) -> &'a str {
    s1
}

/**
 * The function below combines lifetimes, traits and generics. Let's go through
 *
 * First off, notice the generic type T which is given to announcement. Inside
 * of this function we are going to print T so we should ensure that it
 * implements the trait Display. Here, that is done using the where clause
 *
 * Next, note that we are going to return one of x or y, depending on which
 * string slice is longer. The lifetime elision rules here will not be able to
 * infer the lifetime of the return type so we have to specify the lifetimes
 * manually
 */
fn putting_it_all_together<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    prevent_dangling_ref();
    println!();

    let result = longest("hello", "hi");
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    let mut result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());

        // This is valid because string2 is still valid
        println!("The longest string is {}", result);
    }

    // The lifetime of result only lives as long as the shorter of the
    // lifetimes of string1 and string2. string2 expires in the inner
    // scope, so that is when result expires as well.
    //
    // Therefore, if the line below is uncommented, then the code will not
    // compile
    // println!("The longest string is {}", result);

    {
        result = longest2("a", "b");
    }

    // This is totally valid because the lifetime of result and string 1
    // have not expired yet
    println!("The longest string is {}", result);

    // Calls putting it all together
    putting_it_all_together("Hello", "Yo", "This is breaking news");
}
