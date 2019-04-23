pub fn run() {
    // loops - used to iterate until a condition is met

    let mut count = 0;

    // infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);

        // break out of the loop at 20, so the infinite loop doesn't just run forever!
        if count == 20 { break; }
    }

    // while loop (fizzbuzz)

    /*
        fizzbuzz
        if val of count is divisible by 3 print fizz
        if val of count is divisible by 5 print buzz
        if val of count is divisible by both 3 and 5 print fizzbuzz
    */

    // reset count to 1
    count = 1;

    while count <= 100 {
        
        // == --> is equal to
        if count % 15 == 0 {
            println!("{} FIZZBUZZ", count);
        } else if count % 3 == 0 {
            println!("{} FIZZ", count);
        } else if count % 5 == 0 {
            println!("{} BUZZ", count);
        } else {
            println!("{}", count);
        }

        // increment count
        count += 1;
    }

    // for range
    for x in 1..100 {
        if x % 15 == 0 {
            println!("{} FIZZBUZZ", x);
        } else if x % 3 == 0 {
            println!("{} FIZZ", x);
        } else if x % 5 == 0 {
            println!("{} BUZZ", x);
        } else {
            println!("{}", x);
        }
    }
}