use std::collections::HashSet;
use std::fmt;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct WordCategory {
    name: &'static str,
    words: &'static [&'static str],
    description: &'static str,
}

impl WordCategory {
    const fn new(name: &'static str, words: &'static [&'static str], description: &'static str) -> Self {
        Self { name, words, description }
    }
}

mod config {
    use super::WordCategory;

    pub const EASY_WORDS: &[&str] = &["CAT", "DOG", "SUN", "CAR", "BOOK", "TREE", "FISH", "BIRD", "HOME", "LOVE"];
    pub const MEDIUM_WORDS: &[&str] = &["COMPUTER", "ELEPHANT", "MOUNTAIN", "RAINBOW", "BICYCLE", "KITCHEN", "LIBRARY", "GARDEN", "PICTURE", "FREEDOM"];

    pub const CATEGORIES: [WordCategory; 2] = [
        WordCategory::new("Easy", EASY_WORDS, "3-4 letter words"),
        WordCategory::new("Medium", MEDIUM_WORDS, "6-8 letter words"),
    ];

    pub const MAX_WRONG_GUESSES: usize = 6;
    pub const MAX_HINTS: usize = 2;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GuessResult {
    Correct,
    Wrong,
    AlreadyGuessed,
    InvalidInput,
}

impl fmt::Display for GuessResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Correct => write!(f, "✅ Good guess!"),
            Self::Wrong => write!(f, "❌ Wrong guess!"),
            Self::AlreadyGuessed => write!(f, "You already guessed that letter!"),
            Self::InvalidInput => write!(f, "Please enter a valid letter!"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameStats {
    pub revealed_letters: usize,
    pub total_letters: usize,
    pub wrong_guesses: usize,
    pub max_wrong: usize,
    pub hints_used: usize,
    pub max_hints: usize,
}

impl fmt::Display for GameStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Progress: {}/{} letters revealed", self.revealed_letters, self.total_letters)?;
        writeln!(f, "Wrong guesses: {}/{}", self.wrong_guesses, self.max_wrong)?;
        write!(f, "Hints used: {}/{}", self.hints_used, self.max_hints)
    }
}

#[derive(Debug)]
struct SimpleRng {
    seed: u64,
}

impl SimpleRng {
    fn new() -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut hasher = DefaultHasher::new();
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .hash(&mut hasher);

        Self { seed: hasher.finish() }
    }

    fn gen_range(&mut self, max: usize) -> usize {

        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        (self.seed as usize) % max
    }
}

#[derive(Debug)]
pub struct HangmanGame {
    word: String,
    guessed_letters: HashSet<char>,
    wrong_guesses: usize,
    max_wrong: usize,
    category: String,
    hints_used: usize,
    max_hints: usize,
    rng: SimpleRng,
}

impl HangmanGame {

    pub fn new(word: &str, category: &str) -> Self {
        Self {
            word: word.to_uppercase(),
            guessed_letters: HashSet::new(),
            wrong_guesses: 0,
            max_wrong: config::MAX_WRONG_GUESSES,
            category: category.to_string(),
            hints_used: 0,
            max_hints: config::MAX_HINTS,
            rng: SimpleRng::new(),
        }
    }

    pub fn display_word(&self) -> String {
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

    pub fn is_won(&self) -> bool {
        self.word.chars().all(|c| self.guessed_letters.contains(&c))
    }

    pub fn is_lost(&self) -> bool {
        self.wrong_guesses >= self.max_wrong
    }

    pub fn make_guess(&mut self, letter: char) -> GuessResult {
        let letter = letter.to_uppercase().next().unwrap_or(letter);

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

    pub fn get_guessed_letters(&self) -> Vec<char> {
        let mut letters: Vec<char> = self.guessed_letters.iter().copied().collect();
        letters.sort_unstable();
        letters
    }

    pub fn get_hint(&mut self) -> String {
        if self.hints_used >= self.max_hints {
            return "No more hints available!".to_string();
        }

        let unguessed: Vec<char> = self.word
            .chars()
            .filter(|c| !self.guessed_letters.contains(c))
            .collect();

        if unguessed.is_empty() {
            return "You've already won!".to_string();
        }

        self.hints_used += 1;

        let index = self.rng.gen_range(unguessed.len());
        let hint_letter = unguessed[index];

        format!(
            "Hint {}/{}: The word contains the letter '{}'",
            self.hints_used, self.max_hints, hint_letter
        )
    }

    pub fn get_stats(&self) -> GameStats {
        let revealed = self.word
            .chars()
            .filter(|c| self.guessed_letters.contains(c))
            .count();

        GameStats {
            revealed_letters: revealed,
            total_letters: self.word.len(),
            wrong_guesses: self.wrong_guesses,
            max_wrong: self.max_wrong,
            hints_used: self.hints_used,
            max_hints: self.max_hints,
        }
    }

    pub fn get_word(&self) -> &str {
        &self.word
    }

    pub fn get_category(&self) -> &str {
        &self.category
    }
}

mod hangman_art {
    const HANGMAN_STAGES: [&str; 7] = [
        "   ____\n   |  |\n   |\n   |\n   |\n___|___",
        "   ____\n   |  |\n   |  O\n   |\n   |\n___|___",
        "   ____\n   |  |\n   |  O\n   |  |\n   |\n___|___",
        "   ____\n   |  |\n   |  O\n   | /|\n   |\n___|___",
        "   ____\n   |  |\n   |  O\n   | /|\\\n   |\n___|___",
        "   ____\n   |  |\n   |  O\n   | /|\\\n   | /\n___|___",
        "   ____\n   |  |\n   |  O\n   | /|\\\n   | / \\\n___|___",
    ];

    pub fn draw(wrong_guesses: usize) {
        if let Some(stage) = HANGMAN_STAGES.get(wrong_guesses) {
            println!("{}", stage);
        }
    }
}

mod ui {
    use super::*;

    pub fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
    }

    pub fn wait_for_enter() {
        print!("Press Enter to continue...");
        io::stdout().flush().unwrap();
        let mut dummy = String::new();
        io::stdin().read_line(&mut dummy).ok();
    }

    pub fn get_input(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_uppercase()
    }

    pub fn display_game_header() {
        println!("=== INTERMEDIATE HANGMAN ===");
    }

    pub fn display_game_state(game: &HangmanGame) {
        display_game_header();
        println!("Category: {} Words", game.get_category());

        hangman_art::draw(game.wrong_guesses);
        println!();

        println!("Word: {}", game.display_word());
        println!("{}", game.get_stats());

        if !game.guessed_letters.is_empty() {
            println!("Guessed letters: {:?}", game.get_guessed_letters());
        }
    }
}

mod word_selector {
    use super::*;

    pub fn get_random_word(words: &[&str]) -> String {
        let mut rng = SimpleRng::new();
        let index = rng.gen_range(words.len());
        words[index].to_string()
    }

    pub fn choose_difficulty() -> &'static WordCategory {
        println!("Choose word difficulty:");
        for (i, category) in config::CATEGORIES.iter().enumerate() {
            println!("{}. {} ({})", i + 1, category.name, category.description);
        }

        loop {
            let input = ui::get_input("");

            match input.as_str() {
                "1" => return &config::CATEGORIES[0],
                "2" => return &config::CATEGORIES[1],
                _ => println!("Please enter 1 or 2"),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Guess(char),
    Hint,
    Quit,
    Invalid,
}

impl From<&str> for Command {
    fn from(input: &str) -> Self {
        match input {
            "HINT" => Self::Hint,
            "QUIT" => Self::Quit,
            s if s.len() == 1 => {
                if let Some(c) = s.chars().next() {
                    Self::Guess(c)
                } else {
                    Self::Invalid
                }
            }
            _ => Self::Invalid,
        }
    }
}

struct GameController {
    game: HangmanGame,
}

impl GameController {
    fn new(game: HangmanGame) -> Self {
        Self { game }
    }

    fn display_intro(&self) {
        println!("\nWelcome to Intermediate Hangman!");
        println!("Category: {} Words", self.game.get_category());
        println!("Word length: {} letters", self.game.get_word().len());
        println!("Available hints: {}", self.game.max_hints);
        println!("Commands: letter to guess, 'hint' for a hint, 'quit' to exit\n");
    }

    fn handle_command(&mut self, command: Command) -> bool {
        match command {
            Command::Guess(letter) => {
                let result = self.game.make_guess(letter);
                match result {
                    GuessResult::Correct => {
                        println!("✅ Good guess! '{}' is in the word!", letter);
                    }
                    GuessResult::Wrong => {
                        println!("❌ Wrong guess! '{}' is not in the word.", letter);
                    }
                    GuessResult::AlreadyGuessed | GuessResult::InvalidInput => {
                        println!("{}", result);
                    }
                }

                if !self.game.is_won() && !self.game.is_lost() {
                    ui::wait_for_enter();
                }
                false
            }
            Command::Hint => {
                let hint = self.game.get_hint();
                println!("💡 {}", hint);
                ui::wait_for_enter();
                false
            }
            Command::Quit => {
                println!("Thanks for playing! The word was: {}", self.game.get_word());
                true
            }
            Command::Invalid => {
                println!("Please enter a letter!");
                ui::wait_for_enter();
                false
            }
        }
    }

    fn check_end_conditions(&self) -> bool {
        if self.game.is_won() {
            println!("\n🎉 Congratulations! You won!");
            println!("The word was: {}", self.game.get_word());
            let stats = self.game.get_stats();
            println!(
                "Final stats: {}/{} wrong guesses, {}/{} hints used",
                stats.wrong_guesses, stats.max_wrong, stats.hints_used, stats.max_hints
            );
            return true;
        }

        if self.game.is_lost() {
            println!("\n💀 Game over! You've been hanged!");
            println!("The word was: {}", self.game.get_word());
            return true;
        }

        false
    }

    fn run(&mut self) {
        self.display_intro();

        loop {
            ui::clear_screen();
            ui::display_game_state(&self.game);

            if self.check_end_conditions() {
                break;
            }

            let input = ui::get_input("\nEnter a letter, 'hint', or 'quit':\n");
            let command = Command::from(input.as_str());

            if self.handle_command(command) {
                break;
            }
        }

        println!("\nThanks for playing!");
    }
}

fn main() {
    println!("=== INTERMEDIATE HANGMAN ===");
    println!("Features: Hints, categories, better organization\n");

    let category = word_selector::choose_difficulty();
    let word = word_selector::get_random_word(category.words);
    let game = HangmanGame::new(&word, category.name);

    let mut controller = GameController::new(game);
    controller.run();
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

    #[test]
    fn test_command_parsing() {
        assert_eq!(Command::from("A"), Command::Guess('A'));
        assert_eq!(Command::from("HINT"), Command::Hint);
        assert_eq!(Command::from("QUIT"), Command::Quit);
        assert_eq!(Command::from("INVALID"), Command::Invalid);
    }

    #[test]
    fn test_game_stats() {
        let mut game = HangmanGame::new("TEST", "Easy");
        game.make_guess('T');
        game.make_guess('E');

        let stats = game.get_stats();
        assert_eq!(stats.revealed_letters, 3); 
        assert_eq!(stats.total_letters, 4);
        assert_eq!(stats.wrong_guesses, 0);
    }
}
