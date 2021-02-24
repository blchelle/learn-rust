// Hashmaps are less frequently used than the vectors or string so they are not
// automatically brought into scope with the prelude
use std::collections::HashMap;

// Notes about hashmaps:
// ---------------------
// Like vectors, Hashmaps have unknown size, so all data for hashmaps is
// allocated on the heap.
//
// Also like vectors: Hashmaps are homogenous, meaning that all the keys in
// a single hashmap must have the same type and all the values in a hashmap
// must have the same type.

fn main() {
    // Hashmap::new() initialized a new hashmap with unknown types for the
    // key value pairs. Once we add the key value pairs below, the compiler
    // can infer that the keys are strings and the values are i32. Without
    // these lines, we would get a compiler error for unknown type
    let mut scores = HashMap::new();

    scores.insert(String::from("Gryffindor"), 10);
    scores.insert(String::from("Hufflepuff"), 30);

    hashmaps_and_collect();

    hashmaps_and_ownership();

    accessing_values();

    update_map();
}

/**
 * Shows off another way to create a hashmap by combining two sets of vectors
 *
 * Question: What happens when the vectors have unequal lengths?
 * Answer: The extra elements on the longer vector will not be included
 *         the hashmap
 */
fn hashmaps_and_collect() {
    let teams = vec![String::from("Red"), String::from("Blue")];
    let scores = vec![10, 50, 100];

    let game_scores: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();
    println!("Scores is {:?}", game_scores);
}

/**
 * When owned values are given to a hashmap, the hashmap takes ownership of
 * the values and the original variable can no longer be used
 *
 * Types that implement the Copy trait are simply copied into the hashmap
 * rather than giving ownership to the hashmap
 */
fn hashmaps_and_ownership() {
    let field_name = String::from("Favourite color");
    let field_value = 10;

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // The following line would throw a compile time error because ownership
    // of field_name has been given to the hashmap
    // === println!("Field Name: {}", field_name); ===

    // But this line will work just fine because field_value is of type i32
    // which implements the Copy trait. So both the map an the original
    // variable can have copies of the value.
    println!("Field Value: {}", field_value);
}

/**
 * Like most languages we can not guarantee that a certain key exists within
 * a hashmap. So when we attempt to get a value by key the result is of type
 * Option<V> because it could be a value or it could be None in the case that
 * the key didn't exist in the hashmap
 */
fn accessing_values() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Hufflepuff"), 30);

    let team_name = String::from("Blue");

    // There is no guarantee that "Blue" is in our hashmap so score is of
    // type Option<i32>
    let score = scores.get(&team_name);

    // We have to check if score exists in the hashmap before we can use it
    match score {
        Some(x) => println!("{} teams score is {:?}", team_name, x),
        None => println!("{} team does not exist in the map", team_name),
    }

    // We can also iterate over all the entries inside of a hashmap
    //
    // Note the & before scores:
    // Without the &, ownership of the values in the map will be given to the
    // key and value variables, which means that score wouldn't be able to be
    // used after the loop.
    // Instead, we have it so that key and value just borrow the values.
    for (key, value) in &scores {
        println!("{}: {}", key, value)
    }

    println!("{:?}", scores.get(&String::from("Blue")));
}

fn update_map() {
    // When updating a value a value for a key in a hashmap you could:
    // - Add the value only if the key doesn't exist
    // - Overwrite the value if the key exists
    // - Keep the old value if the key exist

    // Overwriting a value
    // -------------------
    // To overwrite any existing value or add a value for a key that
    // doesn't exist yes, call 'insert'
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("Overwriting: {:?}", scores);

    // Only add if it doesn't exist yet
    // --------------------------------
    // To only overwrite in the event that a key is empty use the pattern
    // '.entry(key).or_insert(value)'
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("Update if empty: {:?}", scores);

    // Updating a value based on the old value
    // ---------------------------------------
    // To update a value base on the old value we first need to obtain a
    // reference to the value using '.entry()' and then perform the update
    // using the reference
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // Count will get either a reference to the current count value for a
        // key in the map, or it will create one and initialize it to 0.
        //
        // Then it increments the value by 1
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    print!("Updating based on the old {:?}", map)
}
