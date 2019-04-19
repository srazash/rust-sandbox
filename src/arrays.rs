pub fn run() {
    // fixed list where elements are of the same data type

    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // re-assign values, array must be set as MUTable
    numbers[4] = 100;

    println!("{:?}", numbers);

    // get single value
    println!("Single value: {}", numbers[4]);
}