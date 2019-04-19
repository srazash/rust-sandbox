extern crate chrono;

pub fn run() {
    
    let year: i32 = chrono::Utc::now().format("%Y").to_string().parse().unwrap();
            
    future_leap_years(year);

}

// is given year a leap year? print it out if it is
fn calculate_leap_year(year: i32) {

    // convert year from i32 to f32
    let fyear = year as f32;

    if (year % 100 == 0 && fyear % 400.0 == 0.0) || (year % 100 != 0 && fyear % 4.0 == 0.0) {
        println!("{} is a leap year.", year);
    }

}

// calculate leap years for the next 100 years
fn future_leap_years(year: i32) {

    let mut x: i32 = year;

    while x < (year + 100) {
        calculate_leap_year(x);
        x += 1;
    }
}