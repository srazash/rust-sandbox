// functions - used to store blocks of code

pub fn run() {
    greeting("Hi", "Ryan");
    println!("10 + 90 = {}", add(10, 90));

    // bind function to variable
    let get_sum = add (100, 900);
    println!("100 + 900 = {}", get_sum);

    // closure
    let n3: i32 = 20;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C-Sum: 20 + 80 + 20 = {}", add_nums(20, 80));
}

fn greeting(greet: &str, name: &str) {
    println!("{}, nice to meet you {}.", greet, name);
}

// we define a return type (-> i32)
// the statement without an EOL is the return value
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}