use std::env;

fn mockify (str: &str) -> String {    
    let mut mockified: String = String::from("");
    let mut index = 0;

    for char in str.chars() {

        if index % 2 == 0 {
            let lowercase: String = char.to_lowercase().to_string();
            mockified.push_str(&lowercase)
        } else {
            let uppercase: String = char.to_uppercase().to_string();
            mockified.push_str(&uppercase)
        }
        index = index + 1;

    }

    return mockified
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("No argument provided.");
        std::process::exit(1);
    } else {
        let mockified = mockify(&args[1]);
        println!("{}", mockified);
        std::process::exit(0);
    }
}
