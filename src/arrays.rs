pub fn run() {
    use std::mem;
    
    // fixed list where elements are of the same data type
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // re-assign values, array must be set as MUTable to change values!
    numbers[4] = 20;

    println!("{:?}", numbers);

    // get single value
    println!("Single value: {}", numbers[4]);

    // get array length
    println!("Array Length: {}", numbers.len());

    // arrays stack allocated
    println!("Array occupies {} bytes.", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}