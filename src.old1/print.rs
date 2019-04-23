pub fn run() {
    // print to console
    println!("Hello from print.rs!");

    // basic formatting
    println!("Number: {}", 1);
    println!("{} is from {}.", "Ryan", "Yorkshire");

    // positional formatting
    println!("{0} is from {1} and {0} likes to {2}.", "Ryan", "Yorkshire", "code");

    // named arguments
    println!("{name} likes to play {game}.", name = "Ryan", game = "Dark Souls");

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}