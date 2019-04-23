pub fn run() {
    /*
    Primitive Types:
    Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
    (number of bits they take in memory)
    u = unsigned ints ---> no negative values
    i = regular ints ---> can be negative or positive
    Floats: f32, f64
    Boolean (bool)
    Character (char)
    Tuples
    Arrays
    */

    // Rust is a statically typed language BUT the compiler can uually infer the types at compile time

    let x = 1; // default i32
    let y = 2.5; // default f64

    let z: i64 = 250000; // set type as 64-bit integer

    // find max size
    println!(" Max i8: {}", std::i8::MAX);
    println!(" Max i16: {}", std::i16::MAX);
    println!(" Max i32: {}", std::i32::MAX);
    println!(" Max i64: {}", std::i64::MAX);
    println!(" Max i128: {}", std::i128::MAX);

    // boolean
    let is_active: bool = true;

    // get boolean from expression
    // compiler *should* infer this is type BOOLEAN
    // is_greater *should* be TRUE because 10 is greater than 5
    let is_greater = 10 > 5;

    println!("{:?}", (x, y, z, is_active, is_greater));

    // characters, must be in single quotes and must be a single character
    let char1 = 'a';

    // can use unicode shortcodes
    let face = '\u{1F600}';

    println!("{:?}", (char1, face));
}