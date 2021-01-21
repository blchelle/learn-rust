fn main() {
    // Loop that breaks after 5 iterations
    let mut counter = 0;
    loop {
        println!("The value of counter is {}", counter);
        counter += 1;

        if counter > 5 {
            break;
        }
    }

    // Loop that breaks after 5 iterations,
    // then returns 2 times the count
    counter = 0;
    let result = loop {
        counter += 1;

        if counter > 5 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // While loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Loop through a collection with while
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("the value at index {} of a is {}", index, a[index]);
        index += 1;
    }

    // Loop through a collection with for
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // Countdown from 3 with for
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
