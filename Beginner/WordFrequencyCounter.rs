use std::collections::HashMap;

fn beginner_word_count() {
    let text = "hello world hello rust world rust programming";
    let mut word_count = HashMap::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    
    for word in words {
        let count = word_count.get(word).unwrap_or(&0);
        word_count.insert(word, count + 1);
    }
    
    println!("=== BEGINNER VERSION ===");
    for (word, count) in &word_count {
        println!("{}: {}", word, count);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    beginner_word_count();
    Ok(())
}
