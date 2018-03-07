use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    check_if_domain_is_in_preferred_name_syntax(query)
}

fn check_if_domain_is_in_preferred_name_syntax(domain: &String){
    println!("domain is: {}", domain);
    let mut split = domain.split(".");
    for s in split {
        println!("{}", s);     
    }
}
