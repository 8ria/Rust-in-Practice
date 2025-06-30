use std::collections::{HashSet, HashMap};
use std::hash::Hash;

trait EmojiClassifier {
    fn classify(&self, emoji: char) -> EmojiCategory;
    fn get_complexity_weight(&self, category: &EmojiCategory) -> u32;
}

trait PasswordValidator<T> {
    fn validate(&self, input: T) -> ValidationResult;
}

trait PatternAnalyzer {
    fn analyze_patterns(&self, emojis: &[char]) -> PatternScore;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum EmojiCategory {
    Face,
    Object, 
    Nature,
    Animal,
    Food,
    Unknown,
}

#[derive(Clone)] 
struct AdvancedEmojiClassifier {
    classification_map: HashMap<char, EmojiCategory>,
    complexity_weights: HashMap<EmojiCategory, u32>,
}

impl AdvancedEmojiClassifier {
    fn new() -> Self {
        let mut map = HashMap::new();
        let mut weights = HashMap::new();

        let face_emojis = ['😀', '😎', '🤔', '😊', '🙄', '😱', '🤗', '😴'];
        let object_emojis = ['🔑', '🎯', '⚡', '🔒', '💎', '🛡', '⭐', '🎪'];
        let nature_emojis = ['🌟', '🌈', '🔥', '🌙', '☀', '🌊', '🌸', '🍃'];
        let animal_emojis = ['🦄', '🐱', '🦊', '🐺', '🦅', '🐢', '🦋', '🐝'];
        let food_emojis = ['🍎', '🍕', '🎂', '🍯', '🥑', '🍓', '🥨', '🍜'];

        for &emoji in &face_emojis { map.insert(emoji, EmojiCategory::Face); }
        for &emoji in &object_emojis { map.insert(emoji, EmojiCategory::Object); }
        for &emoji in &nature_emojis { map.insert(emoji, EmojiCategory::Nature); }
        for &emoji in &animal_emojis { map.insert(emoji, EmojiCategory::Animal); }
        for &emoji in &food_emojis { map.insert(emoji, EmojiCategory::Food); }

        weights.insert(EmojiCategory::Face, 1);
        weights.insert(EmojiCategory::Food, 2);
        weights.insert(EmojiCategory::Animal, 3);
        weights.insert(EmojiCategory::Nature, 4);
        weights.insert(EmojiCategory::Object, 5);
        weights.insert(EmojiCategory::Unknown, 1);

        Self {
            classification_map: map,
            complexity_weights: weights,
        }
    }
}

impl EmojiClassifier for AdvancedEmojiClassifier {
    fn classify(&self, emoji: char) -> EmojiCategory {
        self.classification_map.get(&emoji)
            .cloned()
            .unwrap_or(EmojiCategory::Unknown)
    }

    fn get_complexity_weight(&self, category: &EmojiCategory) -> u32 {
        *self.complexity_weights.get(category).unwrap_or(&1)
    }
}

struct PatternScore {
    alternating_bonus: u32,
    repetition_penalty: u32,
    sequence_bonus: u32,
}

#[derive(Clone)] 
struct AdvancedPatternAnalyzer;

impl PatternAnalyzer for AdvancedPatternAnalyzer {
    fn analyze_patterns(&self, emojis: &[char]) -> PatternScore {
        let mut alternating_bonus = 0;
        let mut repetition_penalty = 0;
        let mut sequence_bonus = 0;

        if emojis.len() >= 4 {

            let mut alternating_count = 0;
            for window in emojis.windows(3) {
                if window[0] != window[1] && window[1] != window[2] && window[0] != window[2] {
                    alternating_count += 1;
                }
            }
            alternating_bonus = (alternating_count * 5).min(15);

            let mut consecutive_same = 0;
            for window in emojis.windows(2) {
                if window[0] == window[1] {
                    consecutive_same += 1;
                }
            }
            repetition_penalty = consecutive_same * 5;

            let mut unique_sequences = 0;
            for window in emojis.windows(3) {
                if window[0] != window[1] && window[1] != window[2] && window[0] != window[2] {
                    unique_sequences += 1;
                }
            }
            sequence_bonus = (unique_sequences * 3).min(10);
        }

        PatternScore {
            alternating_bonus,
            repetition_penalty,
            sequence_bonus,
        }
    }
}

struct ValidationResult {
    is_valid: bool,
    feedback: String,
    strength_score: u32,
    detailed_analysis: DetailedAnalysis,
}

struct DetailedAnalysis {
    length_score: u32,
    uniqueness_score: u32,
    category_diversity_score: u32,
    pattern_score: u32,
    complexity_score: u32,
    categories_used: HashSet<EmojiCategory>,
    pattern_details: PatternScore,
}

struct ProEmojiPasswordValidator<C, P>
where
    C: EmojiClassifier,
    P: PatternAnalyzer,
{
    classifier: C,
    pattern_analyzer: P,
    min_length: usize,
    min_unique: usize,
}

impl<C, P> ProEmojiPasswordValidator<C, P>
where
    C: EmojiClassifier,
    P: PatternAnalyzer,
{
    fn new(classifier: C, pattern_analyzer: P) -> Self {
        Self {
            classifier,
            pattern_analyzer,
            min_length: 4,
            min_unique: 2,
        }
    }

    fn calculate_detailed_score(&self, emojis: &[char], unique_emojis: &HashSet<char>) -> DetailedAnalysis {

        let length_score = if emojis.len() >= 8 {
            40  
        } else if emojis.len() >= 4 {
            20
        } else {
            0
        };

        let uniqueness_score = if unique_emojis.len() >= 6 {
            50  
        } else if unique_emojis.len() >= 3 {
            30
        } else {
            0
        };

        let categories: HashSet<EmojiCategory> = emojis.iter()
            .map(|&emoji| self.classifier.classify(emoji))
            .collect();
        let category_diversity_score = (categories.len() as u32) * 10;

        let pattern_score_data = self.pattern_analyzer.analyze_patterns(emojis);
        let pattern_score = (pattern_score_data.alternating_bonus 
            + pattern_score_data.sequence_bonus)
            .saturating_sub(pattern_score_data.repetition_penalty);

        let complexity_score = emojis.iter()
            .map(|&emoji| {
                let category = self.classifier.classify(emoji);
                self.classifier.get_complexity_weight(&category)
            })
            .sum::<u32>()
            .min(20);

        DetailedAnalysis {
            length_score,
            uniqueness_score,
            category_diversity_score,
            pattern_score,
            complexity_score,
            categories_used: categories,
            pattern_details: pattern_score_data,
        }
    }

    fn generate_advanced_feedback(&self, analysis: &DetailedAnalysis, 
                                total_score: u32, is_valid: bool) -> String {
        if is_valid {
            format!("✅ Strong emoji password! Score: {}/100", total_score)
        } else if analysis.length_score == 0 {
            "❌ Password too short! Use at least 4 emojis.".to_string()
        } else if analysis.uniqueness_score == 0 {
            "❌ Use more variety! Need at least 2 different emojis.".to_string()
        } else {
            format!("⚠️ Weak password. Score: {}/100. Add more diversity!", total_score)
        }
    }

    fn print_advanced_analysis(&self, analysis: &DetailedAnalysis) {
        println!("🔬 Advanced Analysis:");
        println!("   Length Score: {}/40", analysis.length_score);
        println!("   Uniqueness Score: {}/50", analysis.uniqueness_score);
        println!("   Category Diversity: {}/50", analysis.category_diversity_score);
        println!("   Pattern Score: {}", analysis.pattern_score);
        println!("   Complexity Score: {}/20", analysis.complexity_score);
        println!("   Categories: {:?}", analysis.categories_used);
        println!("   Pattern Details:");
        println!("     - Alternating Bonus: {}", analysis.pattern_details.alternating_bonus);
        println!("     - Repetition Penalty: {}", analysis.pattern_details.repetition_penalty);
        println!("     - Sequence Bonus: {}", analysis.pattern_details.sequence_bonus);
    }
}

impl<C, P> PasswordValidator<&str> for ProEmojiPasswordValidator<C, P>
where
    C: EmojiClassifier,
    P: PatternAnalyzer,
{
    fn validate(&self, password: &str) -> ValidationResult {
        let emojis: Vec<char> = password.chars().collect();
        let unique_emojis: HashSet<char> = emojis.iter().cloned().collect();

        let analysis = self.calculate_detailed_score(&emojis, &unique_emojis);

        let total_score = (analysis.length_score 
            + analysis.uniqueness_score 
            + analysis.category_diversity_score 
            + analysis.complexity_score).min(100);

        let is_valid = emojis.len() >= self.min_length 
            && unique_emojis.len() >= self.min_unique 
            && total_score >= 40;

        let feedback = self.generate_advanced_feedback(&analysis, total_score, is_valid);

        ValidationResult {
            is_valid,
            feedback,
            strength_score: total_score,
            detailed_analysis: analysis,
        }
    }
}

fn main() {
    println!("🎮 EMOJI PASSWORD CHECKER - PRO LEVEL ⚡");
    println!("{}", "=".repeat(50));
    println!("Learning: Traits, generics, advanced patterns, extensible architecture");
    println!();

    let classifier = AdvancedEmojiClassifier::new();
    let pattern_analyzer = AdvancedPatternAnalyzer;
    let validator = ProEmojiPasswordValidator::new(classifier.clone(), pattern_analyzer.clone()); 

    let test_passwords = vec![
        "😀😎🤔🔑",              
        "🔥🔥🔥",                
        "😀😎🤔🔑🌟🌈⚡🎯",      
        "🌟",                    
        "😀😀😀😀😀😀",          
        "🦄🍎🔑😊🌟",            
        "😀🔑😀🔑😀🔑",          
    ];

    println!("Testing emoji passwords with advanced analysis:");
    for (i, password) in test_passwords.iter().enumerate() {
        let result = validator.validate(password);
        println!("{}. Password: {} | Valid: {} | Score: {}", 
                 i + 1, password, result.is_valid, result.strength_score);
        println!("   {}", result.feedback);
        validator.print_advanced_analysis(&result.detailed_analysis);
        println!();
    }

    println!("🎯 Emoji Complexity Weights (for advanced scoring):");
    for category in [EmojiCategory::Face, EmojiCategory::Food, EmojiCategory::Animal, 
                     EmojiCategory::Nature, EmojiCategory::Object] {
        let weight = classifier.get_complexity_weight(&category);
        println!("   {:?}: {} points", category, weight);
    }
    println!();

    println!("🔧 Extensible Architecture Demo:");
    println!("   • Different classifiers can be plugged in");
    println!("   • Pattern analyzers are swappable");
    println!("   • Validator works with any input type T");
    println!("   • Easy to add new emoji categories or rules");
    println!();

    println!("🚀 Advanced Features:");
    println!("   • Pattern detection (alternating, sequences)");
    println!("   • Complexity weighting by emoji category");
    println!("   • Detailed scoring breakdown");
    println!("   • Trait-based extensible design");
}
