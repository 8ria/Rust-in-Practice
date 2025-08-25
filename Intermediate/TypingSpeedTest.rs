use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::time::Instant;

fn main() {
    let sentences = vec![
        "The quick brown fox jumps over the lazy dog",
        "Rust programming is fun and fast",
        "Practice makes perfect in typing skills",
    ];

    println!("Intermediate Typing Test! Type the sentences as fast as possible.");

    let mut rng = rand::thread_rng();
    let sentence = sentences.choose(&mut rng).unwrap();
    println!("Your sentence: '{}'", sentence);

    let start_time = Instant::now();
    print!("Type here -> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let duration = start_time.elapsed();

    let correct_chars = sentence
        .chars()
        .zip(input.chars())
        .filter(|(c1, c2)| c1 == c2)
        .count();

    let total_chars = sentence.chars().count();
    let accuracy = (correct_chars as f64 / total_chars as f64) * 100.0;
    let wpm = (input.split_whitespace().count() as f64) / (duration.as_secs_f64() / 60.0);

    println!("Time taken: {:.2} seconds", duration.as_secs_f64());
    println!("Words per minute (WPM): {:.2}", wpm);
    println!("Accuracy: {:.2}%", accuracy);
}
