pub fn run() {
    // conditionals - used to check the condition of something and act on that

    let age: u8 = 30;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    println!("Your age is {}.", age);

    // if/else

    /*
        && --> AND statement
        || --> OR statement
    */

    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender says: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender says: Sorry, you have to leave!");
    } else {
        println!("Bartender says: I'll need to see your ID.");
    }

    // shorthand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age? {}", is_of_age);

}