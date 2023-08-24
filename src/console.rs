use std::{io, str::FromStr};

use chrono::NaiveDate;


pub fn get_input<T>(prompt: &str, error_message: &str) -> T
where
    T: FromStr
{
    loop {
        let input = get_string(prompt);

        match input.trim().parse() {
            Ok(input) => return input,
            Err(_) => {
                println!("{}", error_message);
                continue;
            }
        }
    }
}

pub fn get_string(prompt: &str) -> String {
    println!("{}", prompt);

    let mut s = String::new();

    if let Err(e) = io::stdin().read_line(&mut s) {
        panic!("Failed to read a line, {}", e);
    }

    s
}

pub fn get_date() -> NaiveDate {
    loop {
        let input: String = get_string("Enter your birthday (YYYY/mm/dd): ");

        match NaiveDate::parse_from_str(&input.trim(), "%Y/%m/%d") {
            Ok(date) => return date,
            Err(_) => {
                println!("Please enter a valid date in format YYYY/mm/dd");
                continue;
            },
        }
    }
}