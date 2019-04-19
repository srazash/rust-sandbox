pub fn run() {
    // variables hold primitive data or references to data
    // variables are IMMUTABLE by default
    // Rust is a block-coped language

    let name = "Ryan";

    // default IMMUTABLE variable
    //let age = 33;

    // MUTABLE variable
    let mut age = 33;

    age += 1;

    println!("My name is {} and I am {}.", name, age);

    // define CONSTANT
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple variables
    let (my_name, my_age) = ("Ryan", 33);
    println!("{} is {}.", my_name, my_age);
}