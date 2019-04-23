// uses external library (crate) - must have a entry in the cargo.toml file
extern crate chrono;

pub fn run() {
    
    // gets the current time from chrono, formats it down to the year
    // the year is coverted to a string and then parsed as an integer
    let year: i16 = chrono::Utc::now().format("%Y").to_string().parse().unwrap();
    
    // calls the future_leap_years function
    future_leap_years(year);

}

// is given year a leap year? print it out if it is
fn calculate_leap_year(year: i16) {

    // convert year from i16 to f32
    let fyear = year as f32;

    if (year % 100 == 0 && fyear % 400.0 == 0.0) || (year % 100 != 0 && fyear % 4.0 == 0.0) {
        println!("{} is a leap year.", year);
    }

}

// calculate leap years for the next 100 years
fn future_leap_years(year: i16) {

    let mut x: i16 = year;

    while x < (year + 100) {
        calculate_leap_year(x);
        x += 1;
    }
}