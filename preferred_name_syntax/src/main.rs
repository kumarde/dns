#[macro_use] extern crate lazy_static;
extern crate regex;

use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let check = check_if_domain_is_in_preferred_name_syntax(query);
    println!("{}", check);
}

fn check_if_domain_is_in_preferred_name_syntax(domain: &String){
    lazy_static! {
        static ref RE: Regex = Regex::new("^(\*\.)?(\?\.)*([A-Za-z0-9*_-]+\.)*[A-Za-z0-9*_-]*$").unwrap(); 
    }
    RE.is_match(domain)
}
