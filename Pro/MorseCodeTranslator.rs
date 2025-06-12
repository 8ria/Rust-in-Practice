use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
enum MorseError {
    UnsupportedCharacter(char),
    InvalidMorseCode(String),
    EmptyInput,
}

impl fmt::Display for MorseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MorseError::UnsupportedCharacter(ch) => write!(f, "Unsupported character: '{}'", ch),
            MorseError::InvalidMorseCode(code) => write!(f, "Invalid morse code: '{}'", code),
            MorseError::EmptyInput => write!(f, "Empty input provided"),
        }
    }
}

impl std::error::Error for MorseError {}

trait Codec<T: ?Sized, U> {
    type Error;
    fn encode(&self, input: &T) -> Result<U, Self::Error>;
    fn decode(&self, input: &U) -> Result<String, Self::Error>;
}

#[derive(Debug)]
struct MorseCodec {
    encode_table: HashMap<char, &'static str>,
    decode_table: HashMap<&'static str, char>,
}

impl Default for MorseCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl MorseCodec {
    const MORSE_TABLE: &'static [(char, &'static str)] = &[
        ('A', ".-"), ('B', "-..."), ('C', "-.-."), ('D', "-.."), ('E', "."),
        ('F', "..-."), ('G', "--."), ('H', "...."), ('I', ".."), ('J', ".---"),
        ('K', "-.-"), ('L', ".-.."), ('M', "--"), ('N', "-."), ('O', "---"),
        ('P', ".--."), ('Q', "--.-"), ('R', ".-."), ('S', "..."), ('T', "-"),
        ('U', "..-"), ('V', "...-"), ('W', ".--"), ('X', "-..-"), ('Y', "-.--"),
        ('Z', "--.."), ('0', "-----"), ('1', ".----"), ('2', "..---"),
        ('3', "...--"), ('4', "....-"), ('5', "....."), ('6', "-...."),
        ('7', "--..."), ('8', "---.."), ('9', "----."),
    ];
    
    fn new() -> Self {
        let encode_table = Self::MORSE_TABLE.iter().cloned().collect();
        let decode_table = Self::MORSE_TABLE.iter().map(|&(c, m)| (m, c)).collect();
        
        Self { encode_table, decode_table }
    }
}

impl Codec<str, String> for MorseCodec {
    type Error = MorseError;
    
    fn encode(&self, input: &str) -> Result<String, Self::Error> {
        if input.is_empty() {
            return Err(MorseError::EmptyInput);
        }
        
        input
            .to_uppercase()
            .chars()
            .map(|ch| match ch {
                ' ' => Ok("/".to_owned()),
                c => self.encode_table
                    .get(&c)
                    .map(|&morse| morse.to_owned())
                    .ok_or(MorseError::UnsupportedCharacter(c)),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|tokens| tokens.join(" "))
    }
    
    fn decode(&self, input: &String) -> Result<String, Self::Error> {
        if input.is_empty() {
            return Err(MorseError::EmptyInput);
        }
        
        input
            .split(" / ")
            .map(|word| {
                word.split_whitespace()
                    .map(|code| {
                        self.decode_table
                            .get(code)
                            .copied()
                            .ok_or_else(|| MorseError::InvalidMorseCode(code.to_owned()))
                    })
                    .collect::<Result<String, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|words| words.join(" "))
    }
}

fn pro_version() {
    println!("\n=== PRO VERSION ===");

    let codec = MorseCodec::new();
    let input = "HELLO WORLD 123";

    let result = codec
        .encode(input)
        .and_then(|morse| {
            println!("Input: {}", input);
            println!("Morse: {}", morse);
            
            codec.decode(&morse).map(|decoded| (morse, decoded))
        });
    
    match result {
        Ok((_morse, decoded)) => {
            println!("Decoded back: {}", decoded);
            println!("Round-trip successful: {}", input == decoded);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn main() {
    pro_version();
}
