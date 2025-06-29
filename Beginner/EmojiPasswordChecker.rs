use std::collections::HashSet;

fn validate_emoji_password(password: &str) -> (bool, String, u32) {
    let emoji_count = password.chars().count();
    let unique_emojis: HashSet<char> = password.chars().collect();
    let unique_count = unique_emojis.len();

    let mut strength_score = 0u32;

    if emoji_count >= 4 {
        strength_score += 20;
    }
    if emoji_count >= 8 {
        strength_score += 20;
    }

    if unique_count >= 3 {
        strength_score += 30;
    }
    if unique_count >= 6 {
        strength_score += 20;
    }

    let has_faces = password.contains('😀') || password.contains('😎') || password.contains('🤔');
    let has_objects = password.contains('🔑') || password.contains('🎯') || password.contains('⚡');
    let has_nature = password.contains('🌟') || password.contains('🌈') || password.contains('🔥');

    let category_count = [has_faces, has_objects, has_nature].iter().filter(|&&x| x).count();
    strength_score += (category_count as u32) * 10;

    strength_score = strength_score.min(100);

    let is_valid = emoji_count >= 4 && unique_count >= 2 && strength_score >= 40;

    let feedback = if is_valid {
        format!("✅ Strong emoji password! Score: {}/100", strength_score)
    } else if emoji_count < 4 {
        "❌ Password too short! Use at least 4 emojis.".to_string()
    } else if unique_count < 2 {
        "❌ Use more variety! Need at least 2 different emojis.".to_string()
    } else {
        format!("⚠️ Weak password. Score: {}/100. Add more diversity!", strength_score)
    };

    (is_valid, feedback, strength_score)
}

fn main() {
    println!("🎮 EMOJI PASSWORD CHECKER - BEGINNER LEVEL 🌱");
    println!("{}", "=".repeat(50));
    println!("Learning: Basic functions, HashSet, simple validation");
    println!();

    let test_passwords = vec![
        "😀😎🤔🔑",              
        "🔥🔥🔥",                
        "😀😎🤔🔑🌟🌈⚡🎯",      
        "🌟",                    
        "😀😀😀😀😀😀",          
        "🎯🔑🌈😎",              
        "🔥🔥🔥🔥",              
    ];

    println!("Testing emoji passwords:");
    for (i, password) in test_passwords.iter().enumerate() {
        let (valid, feedback, score) = validate_emoji_password(password);
        println!("{}. Password: {} | Valid: {} | Score: {}", 
                 i + 1, password, valid, score);
        println!("   {}", feedback);
        println!();
    }

    println!("🎯 Try your own emoji password!");
    println!("Rules:");
    println!("• At least 4 emojis required");
    println!("• At least 2 different emojis needed");
    println!("• Mix faces 😀😎🤔, objects 🔑🎯⚡, and nature 🌟🌈🔥 for higher scores");
    println!();

    println!("Example strong passwords:");
    println!("• 😀🔑🌟⚡ (Score: 70)");
    println!("• 🤔🎯🌈😎🔥💎🌟🔑 (Score: 100)");
}
