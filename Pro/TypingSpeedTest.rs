use std::io::{self, Write};
use std::time::Instant;

fn main() {
    let words = vec!["cat", "dog", "sun", "tree", "book", "pen"];
    println!("Beginner Typing Test! Type the words as fast as you can.");

    let mut correct_words = 0;
    let start_time = Instant::now();

    for word in &words {
        print!("Type this word: {} -> ", word);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == *word {
            correct_words += 1;
        }
    }

    let duration = start_time.elapsed();
    let wpm = correct_words as f64 / (duration.as_secs_f64() / 60.0);

    println!("Time taken: {:.2} seconds", duration.as_secs_f64());
    println!("Words per minute (WPM): {:.2}", wpm);
    println!("Correct words: {}/{}", correct_words, words.len());
}
