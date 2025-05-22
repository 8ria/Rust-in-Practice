use std::io;
use std::fmt;

#[derive(Debug)]
enum ConversionError {
    NumberTooLarge(u64),
    InvalidInput(String),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConversionError::NumberTooLarge(n) => {
                write!(f, "Number {} is too large (max: 999,999,999)", n)
            }
            ConversionError::InvalidInput(s) => {
                write!(f, "Invalid input: '{}'", s)
            }
        }
    }
}

struct NumberConverter {
    max_value: u64,
}

impl NumberConverter {
    fn new() -> Self {
        Self {
            max_value: 999_999_999,
        }
    }

    fn convert(&self, num: u64) -> Result<String, ConversionError> {
        if num > self.max_value {
            return Err(ConversionError::NumberTooLarge(num));
        }

        if num == 0 {
            return Ok("zero".to_string());
        }

        Ok(self.convert_recursive(num))
    }

    fn convert_recursive(&self, num: u64) -> String {
        match num {
            0 => String::new(),
            1..=19 => self.ones_and_teens(num),
            20..=99 => self.convert_tens(num),
            100..=999 => self.convert_hundreds(num),
            1_000..=999_999 => self.convert_thousands(num),
            1_000_000..=999_999_999 => self.convert_millions(num),
            _ => unreachable!("Number should be validated before reaching here"),
        }
    }

    fn ones_and_teens(&self, num: u64) -> String {
        let words = [
            "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
            "seventeen", "eighteen", "nineteen"
        ];
        words[num as usize].to_string()
    }

    fn convert_tens(&self, num: u64) -> String {
        let tens_words = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];

        let tens = num / 10;
        let ones = num % 10;

        if ones == 0 {
            tens_words[tens as usize].to_string()
        } else {
            format!("{}-{}", tens_words[tens as usize], self.ones_and_teens(ones))
        }
    }

    fn convert_hundreds(&self, num: u64) -> String {
        let hundreds = num / 100;
        let remainder = num % 100;

        let mut result = format!("{} hundred", self.ones_and_teens(hundreds));

        if remainder > 0 {
            result.push(' ');
            result.push_str(&self.convert_recursive(remainder));
        }
        result
    }

    fn convert_thousands(&self, num: u64) -> String {
        let thousands = num / 1_000;
        let remainder = num % 1_000;

        let mut result = format!("{} thousand", self.convert_recursive(thousands));

        if remainder > 0 {
            result.push(' ');
            result.push_str(&self.convert_recursive(remainder));
        }
        result
    }

    fn convert_millions(&self, num: u64) -> String {
        let millions = num / 1_000_000;
        let remainder = num % 1_000_000;

        let mut result = format!("{} million", self.convert_recursive(millions));

        if remainder > 0 {
            result.push(' ');
            result.push_str(&self.convert_recursive(remainder));
        }
        result
    }
}

fn get_user_input() -> Result<Option<u64>, ConversionError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| ConversionError::InvalidInput("Failed to read input".to_string()))?;

    let input = input.trim();

    if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
        return Ok(None);
    }

    input.parse::<u64>()
        .map(Some)
        .map_err(|_| ConversionError::InvalidInput(input.to_string()))
}

fn main() {
    println!("=== Number to Words Converter (Intermediate Version) ===");
    println!("Enter a number (0 to 999,999,999) or 'quit' to exit:");

    let converter = NumberConverter::new();

    loop {
        print!("Enter number: ");

        use std::io::Write;
        io::stdout().flush().unwrap();

        match get_user_input() {
            Ok(Some(number)) => {
                match converter.convert(number) {
                    Ok(words) => println!("{} -> \"{}\"", number, words),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Ok(None) => {
                println!("Goodbye!");
                break;
            }
            Err(e) => println!("Error: {}", e),
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_converter() -> NumberConverter {
        NumberConverter::new()
    }

    #[test]
    fn test_basic_conversions() {
        let converter = setup_converter();

        assert_eq!(converter.convert(0).unwrap(), "zero");
        assert_eq!(converter.convert(1).unwrap(), "one");
        assert_eq!(converter.convert(15).unwrap(), "fifteen");
        assert_eq!(converter.convert(42).unwrap(), "forty-two");
    }

    #[test]
    fn test_large_numbers() {
        let converter = setup_converter();

        assert_eq!(converter.convert(1_234_567).unwrap(), 
                   "one million two hundred thirty-four thousand five hundred sixty-seven");
    }

    #[test]
    fn test_error_handling() {
        let converter = setup_converter();

        match converter.convert(1_000_000_000) {
            Err(ConversionError::NumberTooLarge(_)) => (),
            _ => panic!("Expected NumberTooLarge error"),
        }
    }

    #[test]
    fn test_edge_cases() {
        let converter = setup_converter();

        assert_eq!(converter.convert(100).unwrap(), "one hundred");
        assert_eq!(converter.convert(1_000).unwrap(), "one thousand");
        assert_eq!(converter.convert(1_000_000).unwrap(), "one million");
    }
}
