use std::fmt::Display;
use std::io::{self, Write};

#[derive(Debug)]
enum QuizError {
    IoError(io::Error),
    InvalidInput(String),
}

impl From<io::Error> for QuizError {
    fn from(error: io::Error) -> Self {
        QuizError::IoError(error)
    }
}

impl Display for QuizError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuizError::IoError(e) => write!(f, "IO Error: {}", e),
            QuizError::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
        }
    }
}

type Result<T> = std::result::Result<T, QuizError>;

trait Displayable {
    fn display(&self) -> String;
}

trait Gradeable {
    fn grade(&self) -> f64;
    fn performance_message(&self) -> &'static str;
}

trait Answerable {
    fn check_answer(&self, input: &str) -> bool;
}

#[derive(Debug, Clone)]
struct Question {
    id: usize,
    text: String,
    options: Vec<String>,
    correct_answers: Vec<String>,
    correct_option: char,
}

impl Question {
    fn new(
        id: usize,
        text: impl Into<String>,
        options: impl IntoIterator<Item = impl Into<String>>,
        correct_answers: impl IntoIterator<Item = impl Into<String>>,
        correct_option: char,
    ) -> Self {
        Self {
            id,
            text: text.into(),
            options: options.into_iter().map(Into::into).collect(),
            correct_answers: correct_answers.into_iter().map(Into::into).collect(),
            correct_option,
        }
    }
}

impl Displayable for Question {
    fn display(&self) -> String {
        let mut output = format!("{}. {}\n", self.id, self.text);
        for (i, option) in self.options.iter().enumerate() {
            output.push_str(&format!("{}) {}\n", (b'a' + i as u8) as char, option));
        }
        output
    }
}

impl Answerable for Question {
    fn check_answer(&self, input: &str) -> bool {
        let normalized_input = input.trim().to_lowercase();
        
        normalized_input == self.correct_option.to_string() 
            || self.correct_answers
                .iter()
                .any(|answer| normalized_input == answer.to_lowercase())
    }
}

#[derive(Debug)]
struct QuizSession<T: Answerable + Displayable> {
    questions: Vec<T>,
    score: usize,
    answers: Vec<String>,
}

impl<T: Answerable + Displayable> QuizSession<T> {
    fn new(questions: Vec<T>) -> Self {
        let answers = Vec::with_capacity(questions.len());
        Self {
            questions,
            score: 0,
            answers,
        }
    }
    
    fn execute(&mut self) -> Result<()> {
        self.display_header();
        
        for index in 0..self.questions.len() {
            self.process_question_by_index(index)?;
        }
        
        self.display_results();
        Ok(())
    }
    
    fn display_header(&self) {
        println!("=== Rust Quiz App ===");
        println!("Answer the following questions:\n");
    }
    
    fn process_question_by_index(&mut self, index: usize) -> Result<()> {
        print!("{}", self.questions[index].display());
        
        let user_input = self.get_validated_input()?;
        
        if self.questions[index].check_answer(&user_input) {
            println!("Correct! ✓\n");
            self.score += 1;
        } else {
            println!("Wrong! The correct answer is {}. ✗\n", 
                     self.get_correct_answer_display(index));
        }
        
        self.answers.push(user_input);
        Ok(())
    }
    
    fn get_validated_input(&self) -> Result<String> {
        print!("Your answer: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(QuizError::InvalidInput("Empty input not allowed".to_string()));
        }
        
        Ok(trimmed.to_string())
    }
    
    fn get_correct_answer_display(&self, index: usize) -> String {
        match index {
            0 => "Paris".to_string(),
            1 => "8".to_string(),
            2 => "Mercury".to_string(),
            _ => "Unknown".to_string(),
        }
    }
    
    fn display_results(&self) {
        println!("=== Quiz Complete ===");
        println!("Your score: {}/{}", self.score, self.questions.len());
        
        let percentage = self.grade();
        println!("Percentage: {:.1}%", percentage);
        println!("{}", self.performance_message());
    }
}

impl<T: Answerable + Displayable> Gradeable for QuizSession<T> {
    fn grade(&self) -> f64 {
        if self.questions.is_empty() {
            return 0.0;
        }
        (self.score as f64 / self.questions.len() as f64) * 100.0
    }
    
    fn performance_message(&self) -> &'static str {
        match self.grade() {
            p if p >= 80.0 => "Excellent work! 🎉",
            p if p >= 60.0 => "Good job! 👍",
            _ => "Keep practicing! 📚",
        }
    }
}

struct QuizBuilder {
    questions: Vec<Question>,
}

impl QuizBuilder {
    fn new() -> Self {
        Self {
            questions: Vec::new(),
        }
    }
    
    fn add_question(
        mut self,
        text: impl Into<String>,
        options: impl IntoIterator<Item = impl Into<String>>,
        correct_answers: impl IntoIterator<Item = impl Into<String>>,
        correct_option: char,
    ) -> Self {
        let id = self.questions.len() + 1;
        self.questions.push(Question::new(
            id,
            text,
            options,
            correct_answers,
            correct_option,
        ));
        self
    }
    
    fn build(self) -> QuizSession<Question> {
        QuizSession::new(self.questions)
    }
}

fn run_quiz() -> Result<()> {
    let mut quiz = QuizBuilder::new()
        .add_question(
            "What is the capital of France?",
            ["London", "Paris", "Berlin"],
            ["Paris"],
            'b',
        )
        .add_question(
            "What is 5 + 3?",
            ["7", "8", "9"],
            ["8"],
            'b',
        )
        .add_question(
            "Which planet is closest to the Sun?",
            ["Venus", "Mercury", "Earth"],
            ["Mercury"],
            'b',
        )
        .build();
    
    quiz.execute()
}

fn main() {
    if let Err(e) = run_quiz() {
        eprintln!("Quiz failed: {}", e);
        std::process::exit(1);
    }
}
