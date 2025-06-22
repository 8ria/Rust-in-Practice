use std::io;
use std::collections::HashSet;

const EASY_WORDS: &[&str] = &["CAT", "DOG", "SUN", "CAR", "BOOK", "TREE", "FISH", "BIRD", "HOME", "LOVE"];
const MEDIUM_WORDS: &[&str] = &["COMPUTER", "ELEPHANT", "MOUNTAIN", "RAINBOW", "BICYCLE", "KITCHEN", "LIBRARY", "GARDEN", "PICTURE", "FREEDOM"];

#[derive(Debug)]
struct HangmanGame {
    word: String,
    guessed_letters: HashSet<char>,
    wrong_guesses: usize,
    max_wrong: usize,
    category: String,
    hints_used: usize,
    max_hints: usize,
}

#[derive(Debug, PartialEq)]
enum GuessResult {
    Correct,
    Wrong,
    AlreadyGuessed,
    InvalidInput,
}

impl HangmanGame {
    fn new(word: &str, category: &str) -> Self {
        HangmanGame {
            word: word.to_uppercase(),
            guessed_letters: HashSet::new(),
            wrong_guesses: 0,
            max_wrong: 6,
            category: category.to_string(),
            hints_used: 0,
            max_hints: 2,
        }
    }
    
    fn display_word(&self) -> String {
        self.word
            .chars()
            .map(|c| {
                if self.guessed_letters.contains(&c) {
                    format!("{} ", c)
                } else {
                    "_ ".to_string()
                }
            })
            .collect()
    }
    
    fn is_won(&self) -> bool {
        self.word.chars().all(|c| self.guessed_letters.contains(&c))
    }
    
    fn is_lost(&self) -> bool {
        self.wrong_guesses >= self.max_wrong
    }
    
    fn make_guess(&mut self, letter: char) -> GuessResult {
        if !letter.is_alphabetic() {
            return GuessResult::InvalidInput;
        }
        
        if self.guessed_letters.contains(&letter) {
            return GuessResult::AlreadyGuessed;
        }
        
        self.guessed_letters.insert(letter);
        
        if self.word.contains(letter) {
            GuessResult::Correct
        } else {
            self.wrong_guesses += 1;
            GuessResult::Wrong
        }
    }
    
    fn get_guessed_letters(&self) -> Vec<char> {
        let mut letters: Vec<char> = self.guessed_letters.iter().cloned().collect();
        letters.sort();
        letters
    }
    
    fn get_hint(&mut self) -> String {
        if self.hints_used >= self.max_hints {
            return "No more hints available!".to_string();
        }
        
        let unguessed: Vec<char> = self.word.chars()
            .filter(|c| !self.guessed_letters.contains(c))
            .collect();
        
        if unguessed.is_empty() {
            return "You've already won!".to_string();
        }
        
        self.hints_used += 1;
        
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
        let index = (hasher.finish() as usize) % unguessed.len();
        let hint_letter = unguessed[index];
        
        format!("Hint {}/{}: The word contains the letter '{}'", 
                self.hints_used, self.max_hints, hint_letter)
    }
    
    fn get_progress(&self) -> (usize, usize) {
        let revealed = self.word.chars().filter(|c| self.guessed_letters.contains(c)).count();
        (revealed, self.word.len())
    }
}

fn get_random_word(words: &[&str]) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
    let index = (hasher.finish() as usize) % words.len();
    words[index].to_string()
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

fn choose_difficulty() -> &'static [&'static str] {
    println!("Choose word difficulty:");
    println!("1. Easy (3-4 letter words)");
    println!("2. Medium (6-8 letter words)");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim() {
            "1" => return EASY_WORDS,
            "2" => return MEDIUM_WORDS,
            _ => println!("Please enter 1 or 2"),
        }
    }
}

fn main() {
    println!("=== INTERMEDIATE HANGMAN ===");
    println!("Features: Hints, categories, better organization\n");
    
    let word_list = choose_difficulty();
    let word = get_random_word(word_list);
    let category = if word_list.len() == EASY_WORDS.len() { "Easy" } else { "Medium" };
    let mut game = HangmanGame::new(&word, category);
    
    println!("\nWelcome to Intermediate Hangman!");
    println!("Category: {} Words", game.category);
    println!("Word length: {} letters", word.len());
    println!("Available hints: {}", game.max_hints);
    println!("Commands: letter to guess, 'hint' for a hint, 'quit' to exit\n");
    
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("=== INTERMEDIATE HANGMAN ===");
        println!("Category: {} Words", game.category);
        
        draw_hangman(game.wrong_guesses);
        println!();
        
        println!("Word: {}", game.display_word());
        let (revealed, total) = game.get_progress();
        println!("Progress: {}/{} letters revealed", revealed, total);
        println!("Wrong guesses: {}/{}", game.wrong_guesses, game.max_wrong);
        println!("Hints used: {}/{}", game.hints_used, game.max_hints);
        
        if !game.guessed_letters.is_empty() {
            println!("Guessed letters: {:?}", game.get_guessed_letters());
        }
        
        if game.is_won() {
            println!("\n🎉 Congratulations! You won!");
            println!("The word was: {}", game.word);
            println!("Final stats: {}/{} wrong guesses, {}/{} hints used", 
                    game.wrong_guesses, game.max_wrong, game.hints_used, game.max_hints);
            break;
        }
        
        if game.is_lost() {
            println!("\n💀 Game over! You've been hanged!");
            println!("The word was: {}", game.word);
            break;
        }
        
        println!("\nEnter a letter, 'hint', or 'quit':");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_uppercase();
        
        match input.as_str() {
            "HINT" => {
                let hint = game.get_hint();
                println!("💡 {}", hint);
                print!("Press Enter to continue...");
                let mut _dummy = String::new();
                io::stdin().read_line(&mut _dummy).ok();
                continue;
            }
            "QUIT" => {
                println!("Thanks for playing! The word was: {}", game.word);
                break;
            }
            _ => {}
        }
        
        if let Some(letter) = input.chars().next() {
            match game.make_guess(letter) {
                GuessResult::Correct => {
                    println!("✅ Good guess! '{}' is in the word!", letter);
                }
                GuessResult::Wrong => {
                    println!("❌ Wrong guess! '{}' is not in the word.", letter);
                }
                GuessResult::AlreadyGuessed => {
                    println!("You already guessed that letter!");
                }
                GuessResult::InvalidInput => {
                    println!("Please enter a valid letter!");
                }
            }
            
            if !game.is_won() && !game.is_lost() {
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
    fn test_game_creation() {
        let game = HangmanGame::new("TEST", "Easy");
        assert_eq!(game.word, "TEST");
        assert_eq!(game.wrong_guesses, 0);
        assert_eq!(game.max_wrong, 6);
        assert!(!game.is_won());
        assert!(!game.is_lost());
    }
    
    #[test]
    fn test_correct_guess() {
        let mut game = HangmanGame::new("TEST", "Easy");
        let result = game.make_guess('T');
        assert_eq!(result, GuessResult::Correct);
        assert!(game.guessed_letters.contains(&'T'));
        assert_eq!(game.wrong_guesses, 0);
    }
    
    #[test]
    fn test_wrong_guess() {
        let mut game = HangmanGame::new("TEST", "Easy");
        let result = game.make_guess('X');
        assert_eq!(result, GuessResult::Wrong);
        assert!(game.guessed_letters.contains(&'X'));
        assert_eq!(game.wrong_guesses, 1);
    }
    
    #[test]
    fn test_win_condition() {
        let mut game = HangmanGame::new("HI", "Easy");
        game.make_guess('H');
        game.make_guess('I');
        assert!(game.is_won());
    }
}
