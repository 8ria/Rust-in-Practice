use std::io;

fn main() {
    println!("=== Rust Quiz App ===");
    println!("Answer the following questions:\n");
    
    let mut score = 0;
    let total_questions = 3;
    
    println!("1. What is the capital of France?");
    println!("a) London");
    println!("b) Paris");
    println!("c) Berlin");
    print!("Your answer: ");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let answer1 = input.trim().to_lowercase();
    
    if answer1 == "b" || answer1 == "paris" {
        println!("Correct! ✓\n");
        score += 1;
    } else {
        println!("Wrong! The correct answer is Paris. ✗\n");
    }
    
    println!("2. What is 5 + 3?");
    println!("a) 7");
    println!("b) 8");
    println!("c) 9");
    print!("Your answer: ");
    
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let answer2 = input2.trim().to_lowercase();
    
    if answer2 == "b" || answer2 == "8" {
        println!("Correct! ✓\n");
        score += 1;
    } else {
        println!("Wrong! The correct answer is 8. ✗\n");
    }
    
    println!("3. Which planet is closest to the Sun?");
    println!("a) Venus");
    println!("b) Mercury");
    println!("c) Earth");
    print!("Your answer: ");
    
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read line");
    let answer3 = input3.trim().to_lowercase();
    
    if answer3 == "b" || answer3 == "mercury" {
        println!("Correct! ✓\n");
        score += 1;
    } else {
        println!("Wrong! The correct answer is Mercury. ✗\n");
    }
    
    println!("=== Quiz Complete ===");
    println!("Your score: {}/{}", score, total_questions);
    
    let percentage = (score as f64 / total_questions as f64) * 100.0;
    println!("Percentage: {:.1}%", percentage);
    
    if percentage >= 80.0 {
        println!("Excellent work! 🎉");
    } else if percentage >= 60.0 {
        println!("Good job! 👍");
    } else {
        println!("Keep practicing! 📚");
    }
}
