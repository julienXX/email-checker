extern crate regex;
use regex::Regex;
use std::env;

fn main() {
    let re = Regex::new(r"^\w+([-+.']\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$").unwrap();

    match env::args().nth(1) {
        Some(date) => {
            if re.is_match(&date) {
                println!("true");
            } else {
                println!("false");
            }
        }
        None => {
            println!("Usage: email-checker <email>");
            return;
        }
    };
}
