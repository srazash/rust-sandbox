pub fn run() {

    // tuple group together values of different types
    // max 12 elements per tuple

    let person: (&str, &str, i8) = ("Ryan", "Yorkshire", 33);

    println!("{} if from {} and is {}.", person.0, person.1, person.2);
}