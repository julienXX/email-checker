extern crate regex;
use regex::Regex;
use std::env;

fn main() {
    println!("Starting email-checker...");

    let re = Regex::new(r"^\w+([-+.']\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$").unwrap();

    match env::args().nth(1) {
        Some(email) => {
            if re.is_match(&email) {
                println!("{} is a valid email.", email);
            } else {
                println!("{} is NOT a valid email.", email);
            }
        }
        None => {
            println!("Usage: email-checker <email>");
            return;
        }
    };
}
