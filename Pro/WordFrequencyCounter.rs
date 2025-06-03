use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, Clone)]
struct WordFrequency<T> {
    frequencies: HashMap<T, usize>,
    total_words: usize,
}

impl<T> WordFrequency<T> 
where
    T: Hash + Eq + Clone,
{
    fn new() -> Self {
        Self {
            frequencies: HashMap::new(),
            total_words: 0,
        }
    }
    
    fn add_word(&mut self, word: T) {
        *self.frequencies.entry(word).or_insert(0) += 1;
        self.total_words += 1;
    }
    
    fn frequency(&self, word: &T) -> f64 {
        self.frequencies.get(word)
            .map(|&count| count as f64 / self.total_words as f64)
            .unwrap_or(0.0)
    }
    
    fn most_frequent(&self, n: usize) -> Vec<(&T, usize)> 
    where
        T: Ord,
    {
        let mut items: Vec<_> = self.frequencies.iter().collect();
        items.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
        items.into_iter().take(n).map(|(k, &v)| (k, v)).collect()
    }
}

impl<T> FromIterator<T> for WordFrequency<T>
where
    T: Hash + Eq + Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut freq = Self::new();
        for word in iter {
            freq.add_word(word);
        }
        freq
    }
}

impl<T> Display for WordFrequency<T>
where
    T: Display + Hash + Eq + Ord,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut items: Vec<_> = self.frequencies.iter().collect();
        items.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
        
        for (word, count) in items {
            writeln!(f, "{}: {}", word, count)?;
        }
        Ok(())
    }
}

fn professional_word_count() -> Result<(), Box<dyn std::error::Error>> {
    let text = "hello world hello rust world rust programming";
    
    let word_freq: WordFrequency<&str> = text
        .split_whitespace()
        .collect();
    
    println!("\n=== PROFESSIONAL VERSION ===");
    print!("{}", word_freq);
    println!("Most frequent words (top 3):");
    for (word, count) in word_freq.most_frequent(3) {
        let freq_percent = word_freq.frequency(word) * 100.0;
        println!("  {}: {} ({:.1}%)", word, count, freq_percent);
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    professional_word_count()?;
    Ok(())
}
