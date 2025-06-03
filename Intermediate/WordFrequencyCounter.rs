use std::collections::HashMap;

fn intermediate_word_count() -> Result<(), Box<dyn std::error::Error>> {
    let text = "hello world hello rust world rust programming";
    
    let word_count: HashMap<&str, usize> = text
        .split_whitespace()
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        });
    
    println!("\n=== INTERMEDIATE VERSION ===");
    let mut sorted_words: Vec<_> = word_count.iter().collect();
    sorted_words.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    
    for (word, count) in sorted_words {
        println!("{}: {}", word, count);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    intermediate_word_count()?;
    Ok(())
}
