use std::io;

fn main() {
    println!("Please input a celsius temperature");

    // Read input
    let mut celsius = String::new();

    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line.");

    let celsius = celsius.trim().parse::<f32>().unwrap();

    let fahrenheit: f32 = transform_to_fahrenheit(celsius);

    println!("{} celsius is {} fahrenheit", celsius, fahrenheit)
}

// Helper function to transform
fn transform_to_fahrenheit(c: f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}
