use std::io::{stdin, stdout, Write};

pub fn input(message: &str) -> String {
    print!("{}", &message);
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
