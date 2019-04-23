pub fn run() {
    // Primitive str is IMMUTABLE fixed-length string in memory
    // String is a MUTABLE, heap-allocated data structure

    let mut hello = String::from("Hello");

    // get length
    println!("Length: {} characters", hello.len());

    hello.push(' '); // push a char
    hello.push_str("World"); // push a string

    println!("Length: {} characters", hello.len());

    println!("{}", hello);

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // check if empty
    println!("Empty string? {}", hello.is_empty());

    // check if string contains a substring
    println!("Contains 'world'? {}", hello.contains("World"));

    // replace a substring
    println!("Replace: {}", hello.replace("World", "There"));

    // loop through a string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // assertion testing
    assert_eq!(3, s.len());

}