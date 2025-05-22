use std::io;

fn main() {
    println!("=== Number to Words Converter (Beginner Version) ===");
    println!("Enter a number (0 to 999) or 'quit' to exit:");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "quit" {
            println!("Goodbye!");
            break;
        }

        match input.parse::<u32>() {
            Ok(number) => {

                if number > 999 {
                    println!("Please enter a number between 0 and 999");
                } else {
                    let words = convert_to_words(number);
                    println!("{} -> {}", number, words);
                }
            }
            Err(_) => {
                println!("That's not a valid number! Please try again.");
            }
        }
        println!(); 
    }
}

fn convert_to_words(num: u32) -> String {
    if num == 0 {
        return "zero".to_string();
    }

    let mut result = String::new();

    if num >= 100 {
        let hundreds = num / 100;
        result.push_str(&get_ones_word(hundreds));
        result.push_str(" hundred");

        if num % 100 != 0 {
            result.push(' ');
        }
    }

    let remainder = num % 100;
    if remainder > 0 {
        result.push_str(&convert_under_hundred(remainder));
    }
    result
}

fn convert_under_hundred(num: u32) -> String {
    if num == 0 {
        return String::new();
    }

    if num <= 19 {
        return get_ones_word(num);
    }

    let tens = num / 10;
    let ones = num % 10;

    let mut result = get_tens_word(tens);

    if ones > 0 {
        result.push('-');
        result.push_str(&get_ones_word(ones));
    }
    result
}

fn get_ones_word(num: u32) -> String {
    let word = match num {
        0 => "",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => "",
    };
    word.to_string()
}

fn get_tens_word(num: u32) -> String {
    let word = match num {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => "",
    };
    word.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_numbers() {
        assert_eq!(convert_to_words(0), "zero");
        assert_eq!(convert_to_words(1), "one");
        assert_eq!(convert_to_words(15), "fifteen");
    }

    #[test]
    fn test_tens() {
        assert_eq!(convert_to_words(20), "twenty");
        assert_eq!(convert_to_words(42), "forty-two");
    }

    #[test]
    fn test_hundreds() {
        assert_eq!(convert_to_words(100), "one hundred");
        assert_eq!(convert_to_words(123), "one hundred twenty-three");
    }
}
