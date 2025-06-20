use std::io;
use std::io::Write;

#[derive(Debug)]
struct Question {
    text: String,
    options: Vec<String>,
    correct_answer: String,
    correct_option: char,
}

impl Question {
    fn new(text: &str, options: Vec<&str>, correct_answer: &str, correct_option: char) -> Self {
        Question {
            text: text.to_string(),
            options: options.iter().map(|s| s.to_string()).collect(),
            correct_answer: correct_answer.to_string(),
            correct_option,
        }
    }
    
    fn display(&self) {
        println!("{}", self.text);
        for (i, option) in self.options.iter().enumerate() {
            println!("{}) {}", (b'a' + i as u8) as char, option);
        }
    }
    
    fn check_answer(&self, user_input: &str) -> bool {
        let input = user_input.trim().to_lowercase();
        input == self.correct_option.to_string() || input == self.correct_answer.to_lowercase()
    }
}

struct Quiz {
    questions: Vec<Question>,
    score: usize,
}

impl Quiz {
    fn new() -> Self {
        let questions = vec![
            Question::new(
                "1. What is the capital of France?",
                vec!["London", "Paris", "Berlin"],
                "Paris",
                'b'
            ),
            Question::new(
                "2. What is 5 + 3?",
                vec!["7", "8", "9"],
                "8",
                'b'
            ),
            Question::new(
                "3. Which planet is closest to the Sun?",
                vec!["Venus", "Mercury", "Earth"],
                "Mercury",
                'b'
            ),
        ];
        
        Quiz { questions, score: 0 }
    }
    
    fn get_user_input() -> String {
        print!("Your answer: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input
    }
    
    fn run(&mut self) {
        println!("=== Rust Quiz App ===");
        println!("Answer the following questions:\n");
        
        for (i, question) in self.questions.iter().enumerate() {
            question.display();
            let user_input = Self::get_user_input();
            
            if question.check_answer(&user_input) {
                println!("Correct! ✓\n");
                self.score += 1;
            } else {
                println!("Wrong! The correct answer is {}. ✗\n", question.correct_answer);
            }
        }
        
        self.display_results();
    }
    
    fn display_results(&self) {
        println!("=== Quiz Complete ===");
        println!("Your score: {}/{}", self.score, self.questions.len());
        
        let percentage = (self.score as f64 / self.questions.len() as f64) * 100.0;
        println!("Percentage: {:.1}%", percentage);
        
        let message = match percentage {
            p if p >= 80.0 => "Excellent work! 🎉",
            p if p >= 60.0 => "Good job! 👍",
            _ => "Keep practicing! 📚",
        };
        
        println!("{}", message);
    }
}

fn main() {
    let mut quiz = Quiz::new();
    quiz.run();
}
