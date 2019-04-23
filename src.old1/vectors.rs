pub fn run() {
    use std::mem;
    
    // expandable list where elements are of the same data type
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // re-assign values, vector must be set as MUTable to change values!
    numbers[4] = 20;

    // add on to vector
    numbers.push(500);
    numbers.push(1000);
    numbers.push(10000);

    // "pop" off the LAST value
    numbers.pop();

    println!("{:?}", numbers);

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop through and mutate vector values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers);

    // get single value
    println!("Single value: {}", numbers[4]);

    // get vector length
    println!("Vector Length: {}", numbers.len());

    // vector stack allocated
    println!("Vector occupies {} bytes.", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}