use std::io::{self, Write};
use std::time::Instant;

fn main() {
    let paragraph = "Rust is a systems programming language that focuses on safety, speed, and concurrency.";
    println!("Pro Typing Test! Type the paragraph exactly as shown.");
    println!("{}\n", paragraph);

    let start_time = Instant::now();
    print!("Type here -> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let duration = start_time.elapsed();

    let mut correct_chars = 0;
    let mut errors = 0;

    for (c1, c2) in paragraph.chars().zip(input.chars()) {
        if c1 == c2 {
            correct_chars += 1;
        } else {
            errors += 1;
        }
    }

    let total_chars = paragraph.chars().count();
    let accuracy = (correct_chars as f64 / total_chars as f64) * 100.0;
    let wpm = (input.split_whitespace().count() as f64) / (duration.as_secs_f64() / 60.0);

    println!("\nTime taken: {:.2} seconds", duration.as_secs_f64());
    println!("Words per minute (WPM): {:.2}", wpm);
    println!("Accuracy: {:.2}%", accuracy);
    println!("Errors: {}", errors);
}
