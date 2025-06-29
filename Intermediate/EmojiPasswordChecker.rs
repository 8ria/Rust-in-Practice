use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum EmojiCategory {
    Face,
    Object,
    Nature,
    Symbol,
    Animal,
    Food,
    Unknown,
}

struct PasswordAnalyzer {
    emoji_categories: HashMap<char, EmojiCategory>,
    min_length: usize,
    min_unique: usize,
}

impl PasswordAnalyzer {
    fn new() -> Self {
        let mut categories = HashMap::new();

        for emoji in ['😀', '😎', '🤔', '😊', '🙄', '😱', '🤗', '😴'] {
            categories.insert(emoji, EmojiCategory::Face);
        }

        for emoji in ['🔑', '🎯', '⚡', '🔒', '💎', '🛡', '⭐', '🎪'] {
            categories.insert(emoji, EmojiCategory::Object);
        }

        for emoji in ['🌟', '🌈', '🔥', '🌙', '☀', '🌊', '🌸', '🍃'] {
            categories.insert(emoji, EmojiCategory::Nature);
        }

        for emoji in ['🦄', '🐱', '🦊', '🐺', '🦅', '🐢', '🦋', '🐝'] {
            categories.insert(emoji, EmojiCategory::Animal);
        }

        for emoji in ['🍎', '🍕', '🎂', '🍯', '🥑', '🍓', '🥨', '🍜'] {
            categories.insert(emoji, EmojiCategory::Food);
        }

        Self {
            emoji_categories: categories,
            min_length: 4,
            min_unique: 2,
        }
    }

    fn categorize_emoji(&self, emoji: char) -> EmojiCategory {
        self.emoji_categories.get(&emoji)
            .cloned()
            .unwrap_or(EmojiCategory::Unknown)
    }

    fn analyze_password(&self, password: &str) -> PasswordResult {
        let emojis: Vec<char> = password.chars().collect();
        let unique_emojis: HashSet<char> = emojis.iter().cloned().collect();

        let categories: HashSet<EmojiCategory> = emojis.iter()
            .map(|&emoji| self.categorize_emoji(emoji))
            .collect();

        let mut score = self.calculate_strength_score(&emojis, &unique_emojis, &categories);

        if self.has_alternating_pattern(&emojis) {
            score += 15;
        }

        let is_valid = emojis.len() >= self.min_length 
            && unique_emojis.len() >= self.min_unique 
            && score >= 40;

        let feedback = self.generate_feedback(is_valid, &emojis, &unique_emojis, score);

        PasswordResult {
            is_valid,
            feedback,
            strength_score: score,
            categories_used: categories,
        }
    }

    fn calculate_strength_score(&self, emojis: &[char], unique_emojis: &HashSet<char>, 
                               categories: &HashSet<EmojiCategory>) -> u32 {
        let mut score = 0u32;

        if emojis.len() >= 4 {
            score += 20;
        }
        if emojis.len() >= 8 {
            score += 20;
        }

        if unique_emojis.len() >= 3 {
            score += 30;
        }
        if unique_emojis.len() >= 6 {
            score += 20;
        }

        score += (categories.len() as u32) * 10;

        score.min(100)
    }

    fn has_alternating_pattern(&self, emojis: &[char]) -> bool {
        if emojis.len() < 4 { return false; }

        let categories: Vec<EmojiCategory> = emojis.iter()
            .map(|&emoji| self.categorize_emoji(emoji))
            .collect();

        for i in 0..categories.len()-2 {
            if categories[i] == categories[i+2] && categories[i] != categories[i+1] {
                return true;
            }
        }
        false
    }

    fn generate_feedback(&self, is_valid: bool, emojis: &[char], 
                       unique_emojis: &HashSet<char>, score: u32) -> String {
        if is_valid {
            format!("✅ Strong emoji password! Score: {}/100", score)
        } else if emojis.len() < self.min_length {
            "❌ Password too short! Use at least 4 emojis.".to_string()
        } else if unique_emojis.len() < self.min_unique {
            "❌ Use more variety! Need at least 2 different emojis.".to_string()
        } else {
            format!("⚠️ Weak password. Score: {}/100. Add more diversity!", score)
        }
    }

    fn print_detailed_analysis(&self, result: &PasswordResult) {
        println!("📊 Detailed Analysis:");
        println!("   Categories used: {:?}", result.categories_used);
        println!("   Total categories: {}", result.categories_used.len());
        if result.categories_used.len() >= 3 {
            println!("   🎯 Great category diversity!");
        }
    }
}

struct PasswordResult {
    is_valid: bool,
    feedback: String,
    strength_score: u32,
    categories_used: HashSet<EmojiCategory>,
}

fn validate_emoji_password(password: &str) -> (bool, String, u32) {
    let analyzer = PasswordAnalyzer::new();
    let result = analyzer.analyze_password(password);
    (result.is_valid, result.feedback, result.strength_score)
}

fn main() {
    println!("🎮 EMOJI PASSWORD CHECKER - INTERMEDIATE LEVEL 🚀");
    println!("{}", "=".repeat(50));
    println!("Learning: Structs, enums, methods, HashMap, organized code");
    println!();

    let analyzer = PasswordAnalyzer::new();

    let test_passwords = vec![
        "😀😎🤔🔑",              
        "🔥🔥🔥",                
        "😀😎🤔🔑🌟🌈⚡🎯",      
        "🌟",                    
        "😀😀😀😀😀😀",          
        "🦄🍎🔑😊",              
        "🐱🦊🐺🦅",              
    ];

    println!("Testing emoji passwords with detailed analysis:");
    for (i, password) in test_passwords.iter().enumerate() {
        let result = analyzer.analyze_password(password);
        println!("{}. Password: {} | Valid: {} | Score: {}", 
                 i + 1, password, result.is_valid, result.strength_score);
        println!("   {}", result.feedback);
        analyzer.print_detailed_analysis(&result);
        println!();
    }

    println!("📋 Available Emoji Categories:");
    println!("😀 Faces: 😀😎🤔😊🙄😱🤗😴");
    println!("🔑 Objects: 🔑🎯⚡🔒💎🛡⭐🎪");
    println!("🌟 Nature: 🌟🌈🔥🌙☀🌊🌸🍃");
    println!("🦄 Animals: 🦄🐱🦊🐺🦅🐢🦋🐝");
    println!("🍎 Food: 🍎🍕🎂🍯🥑🍓🥨🍜");
    println!();

    println!("💡 Pro tip: Mix different categories for higher scores!");
    println!("   Example: 😎🔑🌟🦄🍎 uses 5 different categories!");
}
