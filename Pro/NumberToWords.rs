use std::fmt;
use std::io::{self, Write};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum ConversionError {
    NumberTooLarge { value: u64, max: u64 },
    InvalidInput(String),
    IoError(String),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NumberTooLarge { value, max } => {
                write!(f, "Number {} exceeds maximum allowed value {}", value, max)
            }
            Self::InvalidInput(input) => write!(f, "Invalid input: '{}'", input),
            Self::IoError(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl std::error::Error for ConversionError {}

pub type Result<T> = std::result::Result<T, ConversionError>;

pub trait NumberToWords<T> {
    fn to_words(&self, num: T) -> Result<String>;
    fn max_value(&self) -> T;
}

#[derive(Debug, Clone)]
pub struct ConverterConfig {
    pub max_value: u64,
    pub use_and: bool,  
    pub capitalize_first: bool,
}

impl Default for ConverterConfig {
    fn default() -> Self {
        Self {
            max_value: 999_999_999_999_999_999, 
            use_and: false,
            capitalize_first: false,
        }
    }
}

const SCALE_WORDS: &[&str] = &[
    "", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion"
];

const ONES: &[&str] = &[
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
    "seventeen", "eighteen", "nineteen"
];

const TENS: &[&str] = &[
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"
];

#[derive(Debug, Clone)]
pub struct NumberConverter {
    config: ConverterConfig,
}

impl NumberConverter {
    pub fn new() -> Self {
        Self::with_config(ConverterConfig::default())
    }

    pub fn with_config(config: ConverterConfig) -> Self {
        Self { config }
    }

    pub fn builder() -> ConverterBuilder {
        ConverterBuilder::new()
    }

    fn convert_internal(&self, num: u64) -> Result<String> {
        if num > self.config.max_value {
            return Err(ConversionError::NumberTooLarge {
                value: num,
                max: self.config.max_value,
            });
        }

        if num == 0 {
            return Ok("zero".to_string());
        }

        let result = self.convert_large_number(num);

        if self.config.capitalize_first {
            Ok(Self::capitalize_first_letter(&result))
        } else {
            Ok(result)
        }
    }

    fn convert_large_number(&self, mut num: u64) -> String {
        let mut parts = Vec::new();
        let mut scale_index = 0;

        while num > 0 {
            let group = num % 1000;
            if group > 0 {
                let group_words = self.convert_group_of_three(group);
                if scale_index > 0 {
                    parts.push(format!("{} {}", group_words, SCALE_WORDS[scale_index]));
                } else {
                    parts.push(group_words);
                }
            }
            num /= 1000;
            scale_index += 1;
        }

        parts.reverse();
        parts.join(" ")
    }

    fn convert_group_of_three(&self, num: u64) -> String {
        let mut result = String::new();

        if num >= 100 {
            let hundreds = num / 100;
            result.push_str(ONES[hundreds as usize]);
            result.push_str(" hundred");

            let remainder = num % 100;
            if remainder > 0 {
                if self.config.use_and {
                    result.push_str(" and ");
                } else {
                    result.push(' ');
                }
            }
        }

        let remainder = num % 100;
        if remainder > 0 {
            result.push_str(&self.convert_under_hundred(remainder));
        }
        result
    }

    fn convert_under_hundred(&self, num: u64) -> String {
        match num {
            0 => String::new(),
            1..=19 => ONES[num as usize].to_string(),
            _ => {
                let tens = num / 10;
                let ones = num % 10;

                if ones == 0 {
                    TENS[tens as usize].to_string()
                } else {
                    format!("{}-{}", TENS[tens as usize], ONES[ones as usize])
                }
            }
        }
    }

    fn capitalize_first_letter(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

impl Default for NumberConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl NumberToWords<u64> for NumberConverter {
    fn to_words(&self, num: u64) -> Result<String> {
        self.convert_internal(num)
    }

    fn max_value(&self) -> u64 {
        self.config.max_value
    }
}

pub struct ConverterBuilder {
    config: ConverterConfig,
}

impl ConverterBuilder {
    pub fn new() -> Self {
        Self {
            config: ConverterConfig::default(),
        }
    }

    pub fn max_value(mut self, max: u64) -> Self {
        self.config.max_value = max;
        self
    }

    pub fn use_and(mut self, use_and: bool) -> Self {
        self.config.use_and = use_and;
        self
    }

    pub fn capitalize_first(mut self, capitalize: bool) -> Self {
        self.config.capitalize_first = capitalize;
        self
    }

    pub fn build(self) -> NumberConverter {
        NumberConverter::with_config(self.config)
    }
}

fn get_user_input<T>() -> Result<Option<T>>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| ConversionError::IoError(e.to_string()))?;

    let input = input.trim();

    if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
        return Ok(None);
    }

    input.parse::<T>()
        .map(Some)
        .map_err(|e| ConversionError::InvalidInput(format!("{}: {}", input, e)))
}

pub struct InteractiveSession<T> {
    converter: T,
    prompt: String,
}

impl<T> InteractiveSession<T>
where
    T: NumberToWords<u64>,
{
    pub fn new(converter: T) -> Self {
        Self {
            converter,
            prompt: "Enter number: ".to_string(),
        }
    }

    pub fn with_prompt(mut self, prompt: String) -> Self {
        self.prompt = prompt;
        self
    }

    pub fn run(&self) -> Result<()> {
        println!("=== Number to Words Converter (Pro Version) ===");
        println!("Enter a number (0 to {}) or 'quit' to exit:", self.converter.max_value());

        loop {
            print!("{}", self.prompt);
            io::stdout().flush().map_err(|e| ConversionError::IoError(e.to_string()))?;

            match get_user_input::<u64>()? {
                Some(number) => {
                    match self.converter.to_words(number) {
                        Ok(words) => println!("{} -> \"{}\"", number, words),
                        Err(e) => println!("Error: {}", e),
                    }
                }
                None => {
                    println!("Goodbye!");
                    break;
                }
            }
            println!();
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let converter = NumberConverter::builder()
        .max_value(999_999_999_999)
        .use_and(false)
        .capitalize_first(false)
        .build();

    let session = InteractiveSession::new(converter);
    session.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_numbers() {
        let converter = NumberConverter::new();

        assert_eq!(converter.to_words(0).unwrap(), "zero");
        assert_eq!(converter.to_words(1).unwrap(), "one");
        assert_eq!(converter.to_words(15).unwrap(), "fifteen");
        assert_eq!(converter.to_words(42).unwrap(), "forty-two");
    }

    #[test]
    fn test_hundreds() {
        let converter = NumberConverter::new();

        assert_eq!(converter.to_words(100).unwrap(), "one hundred");
        assert_eq!(converter.to_words(123).unwrap(), "one hundred twenty-three");
        assert_eq!(converter.to_words(500).unwrap(), "five hundred");
    }

    #[test]
    fn test_thousands() {
        let converter = NumberConverter::new();

        assert_eq!(converter.to_words(1_000).unwrap(), "one thousand");
        assert_eq!(converter.to_words(1_234).unwrap(), "one thousand two hundred thirty-four");
        assert_eq!(converter.to_words(12_345).unwrap(), "twelve thousand three hundred forty-five");
    }

    #[test]
    fn test_millions_and_beyond() {
        let converter = NumberConverter::new();

        assert_eq!(converter.to_words(1_000_000).unwrap(), "one million");
        assert_eq!(converter.to_words(1_000_000_000).unwrap(), "one billion");
        assert_eq!(converter.to_words(1_234_567_890).unwrap(), 
                   "one billion two hundred thirty-four million five hundred sixty-seven thousand eight hundred ninety");
    }

    #[test]
    fn test_configuration() {
        let converter = NumberConverter::builder()
            .use_and(true)
            .capitalize_first(true)
            .build();

        assert_eq!(converter.to_words(123).unwrap(), "One hundred and twenty-three");
    }

    #[test]
    fn test_error_conditions() {
        let converter = NumberConverter::builder()
            .max_value(1000)
            .build();

        match converter.to_words(1001) {
            Err(ConversionError::NumberTooLarge { value: 1001, max: 1000 }) => (),
            other => panic!("Expected NumberTooLarge error, got {:?}", other),
        }
    }

    #[test]
    fn test_edge_cases() {
        let converter = NumberConverter::new();

        assert_eq!(converter.to_words(10).unwrap(), "ten");
        assert_eq!(converter.to_words(100).unwrap(), "one hundred");
        assert_eq!(converter.to_words(1_000).unwrap(), "one thousand");

        assert_eq!(converter.to_words(20).unwrap(), "twenty");
        assert_eq!(converter.to_words(200).unwrap(), "two hundred");
        assert_eq!(converter.to_words(2_000).unwrap(), "two thousand");
    }

    #[test]
    fn test_roundtrip_property() {
        let converter = NumberConverter::new();

        for i in 0..1000 {
            let result = converter.to_words(i).unwrap();
            assert!(!result.is_empty() || i == 0);
            if i == 0 {
                assert_eq!(result, "zero");
            } else {
                assert_ne!(result, "zero");
            }
        }
    }
}
