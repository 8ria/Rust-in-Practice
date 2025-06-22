use std::io;

const WORDS: &[&str] = &["CAT", "DOG", "SUN", "CAR", "BOOK", "TREE", "FISH", "BIRD", "HOME", "LOVE"];

fn get_random_word() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
    let index = (hasher.finish() as usize) % WORDS.len();
    WORDS[index].to_string()
}

fn draw_hangman(wrong_guesses: usize) {
    let stages = [
        "   ____\n   |  |\n   |\n   |\n   |\n___|___",
        "   ____\n   |  |\n   |  O\n   |\n   |\n___|___",
        "   ____\n   |  |\n   |  O\n   |  |\n   |\n___|___",
        "   ____\n   |  |\n   |  O\n   | /|\n   |\n___|___",
        "   ____\n   |  |\n   |  O\n   | /|\\\n   |\n___|___",
        "   ____\n   |  |\n   |  O\n   | /|\\\n   | /\n___|___",
        "   ____\n   |  |\n   |  O\n   | /|\\\n   | / \\\n___|___",
    ];
    
    if wrong_guesses < stages.len() {
        println!("{}", stages[wrong_guesses]);
    }
}

fn main() {
    println!("=== BEGINNER HANGMAN ===");
    println!("Welcome to Hangman! Guess the word letter by letter.");
    
    let word = get_random_word();
    let mut guessed_letters = Vec::new();
    let mut wrong_guesses = 0;
    let max_wrong = 6;
    
    println!("The word has {} letters.", word.len());
    
    loop {
        print!("\x1B[2J\x1B[1;1H");
        
        println!("=== BEGINNER HANGMAN ===");
        
        draw_hangman(wrong_guesses);
        println!();
        
        print!("Word: ");
        for c in word.chars() {
            if guessed_letters.contains(&c) {
                print!("{} ", c);
            } else {
                print!("_ ");
            }
        }
        println!("\n");
        
        println!("Wrong guesses: {}/{}", wrong_guesses, max_wrong);
        if !guessed_letters.is_empty() {
            println!("Guessed letters: {:?}", guessed_letters);
        }
        
        let mut all_guessed = true;
        for c in word.chars() {
            if !guessed_letters.contains(&c) {
                all_guessed = false;
                break;
            }
        }
        
        if all_guessed {
            println!("\n🎉 Congratulations! You won!");
            println!("The word was: {}", word);
            break;
        }
        
        if wrong_guesses >= max_wrong {
            println!("\n💀 Game over! You've been hanged!");
            println!("The word was: {}", word);
            break;
        }
        
        println!("\nEnter a letter:");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(_) => {
                println!("Error reading input. Please try again.");
                continue;
            }
        }
        
        let guess = input.trim().to_uppercase().chars().next();
        
        if let Some(letter) = guess {
            if !letter.is_alphabetic() {
                println!("Please enter a valid letter!");
                print!("Press Enter to continue...");
                let mut _dummy = String::new();
                io::stdin().read_line(&mut _dummy).ok();
                continue;
            }
            
            if guessed_letters.contains(&letter) {
                println!("You already guessed that letter!");
                print!("Press Enter to continue...");
                let mut _dummy = String::new();
                io::stdin().read_line(&mut _dummy).ok();
                continue;
            }
            
            guessed_letters.push(letter);
            
            if word.contains(letter) {
                println!("✅ Good guess! '{}' is in the word!", letter);
            } else {
                wrong_guesses += 1;
                println!("❌ Wrong guess! '{}' is not in the word.", letter);
            }
            
            if !all_guessed && wrong_guesses < max_wrong {
                print!("Press Enter to continue...");
                let mut _dummy = String::new();
                io::stdin().read_line(&mut _dummy).ok();
            }
        } else {
            println!("Please enter a letter!");
            print!("Press Enter to continue...");
            let mut _dummy = String::new();
            io::stdin().read_line(&mut _dummy).ok();
        }
    }
    
    println!("\nThanks for playing!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_word_selection() {
        let word = get_random_word();
        assert!(WORDS.contains(&word.as_str()));
        assert!(!word.is_empty());
    }
    
    #[test]
    fn test_word_contains_logic() {
        let word = "TEST";
        assert!(word.contains('T'));
        assert!(!word.contains('X'));
    }
}
