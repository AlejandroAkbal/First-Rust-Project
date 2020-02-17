use std::io;

use url::Url;

fn main() {
    println!("Please input an URL");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // println!("Your input '{}'", input);

    let mut input = Url::parse(&input);

    // let input: u8 = extract_numbers(&input);

    println!("\nResult:\n{}", input.as_str())
}

fn extract_numbers(s: String) -> u8 {
    let result: u8 = 8;
    result
}
