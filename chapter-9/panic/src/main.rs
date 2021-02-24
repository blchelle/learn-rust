fn main() {
    // The line below will cause the program to panic and exit
    // panic!("AHHHHHHHHHHHHHHH");

    // This will also cause the program to panic because we try to access an
    // element beyond the bounds of the vector
    let v = vec![1, 2, 3];
    v[99];
}
