fn main() {
    let run_and_print = |func: fn()| {
        func();
        println!();
    };

    run_and_print(basic_iterators);
    run_and_print(iter_sum);
    run_and_print(iter_maps);
    run_and_print(iter_filter);
}

fn basic_iterators() {
    println!("===== Basic Iterators =====");

    let v1 = vec![1, 2, 3];

    // Up to this point, the code doesn't actually do anything useful
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn iter_sum() {
    println!("===== Iterator Sum =====");

    let v1 = vec![1, 2, 3];
    let sum: i32 = v1.iter().sum();
    println!("Sum: {}", sum);
}

fn iter_maps() {
    println!("===== Iterator Maps =====");

    let v1 = vec![1, 2, 3];

    // .map(...) returns an iterator which doesn't do anything in itself
    // So we have to call collect on the iterator to convert it to a vector
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("V2 is: {:?}", v2);
}

fn iter_filter() {
    println!("===== Iterator Filter =====");

    let v1 = vec![1, 2, 3];

    // .filter(...) returns an iterator which doesn't do anything in itself
    // So we have to call collect on the iterator to convert it to a vector
    let v2: Vec<_> = v1.iter().filter(|x| **x > 1).collect();

    println!("V2 is: {:?}", v2);
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    // Iterates from start to finish in a hardcoded roundabout way
    // The important thing here is to recognize what the 'next' method
    // is doing.
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
