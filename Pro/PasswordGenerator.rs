use std::fmt;
use rand::{CryptoRng, RngCore};

pub trait CharacterSet {
    fn chars(&self) -> &[u8];
    fn is_empty(&self) -> bool {
        self.chars().is_empty()
    }
}

#[derive(Clone, Debug)]
pub struct StandardCharacterSet {
    chars: Vec<u8>,
}

impl StandardCharacterSet {
    pub fn builder() -> CharacterSetBuilder {
        CharacterSetBuilder::new()
    }
}

impl CharacterSet for StandardCharacterSet {
    fn chars(&self) -> &[u8] {
        &self.chars
    }
}

pub struct CharacterSetBuilder {
    chars: Vec<u8>,
}

impl CharacterSetBuilder {
    pub fn new() -> Self {
        Self { chars: Vec::new() }
    }
    
    pub fn uppercase(mut self) -> Self {
        self.chars.extend(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        self
    }
    
    pub fn lowercase(mut self) -> Self {
        self.chars.extend(b"abcdefghijklmnopqrstuvwxyz");
        self
    }
    
    pub fn numbers(mut self) -> Self {
        self.chars.extend(b"0123456789");
        self
    }
    
    pub fn symbols(mut self) -> Self {
        self.chars.extend(b"!@#$%^&*");
        self
    }
    
    pub fn build(self) -> StandardCharacterSet {
        StandardCharacterSet { chars: self.chars }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GeneratorError {
    InvalidLength,
    EmptyCharacterSet,
    InsufficientEntropy,
}

impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "Password length must be greater than 0"),
            Self::EmptyCharacterSet => write!(f, "Character set cannot be empty"),
            Self::InsufficientEntropy => write!(f, "Insufficient entropy for secure generation"),
        }
    }
}

impl std::error::Error for GeneratorError {}

pub struct SecurePasswordGenerator<R: RngCore + CryptoRng> {
    rng: R,
}

impl<R: RngCore + CryptoRng> SecurePasswordGenerator<R> {
    pub fn new(rng: R) -> Self {
        Self { rng }
    }
    
    pub fn generate<C: CharacterSet>(
        &mut self,
        length: usize,
        charset: &C,
    ) -> Result<String, GeneratorError> {
        if length == 0 {
            return Err(GeneratorError::InvalidLength);
        }
        
        if charset.is_empty() {
            return Err(GeneratorError::EmptyCharacterSet);
        }
        
        let chars = charset.chars();
        let mut password = Vec::with_capacity(length);
        
        for _ in 0..length {
            let index = (self.rng.next_u32() as usize) % chars.len();
            password.push(chars[index]);
        }
        
        String::from_utf8(password)
            .map_err(|_| GeneratorError::InsufficientEntropy)
    }
}

fn main() {
    println!("=== Professional Implementation ===");
    let charset = StandardCharacterSet::builder()
        .uppercase()
        .lowercase()
        .numbers()
        .symbols()
        .build();
    
    let mut generator = SecurePasswordGenerator::new(rand::rngs::OsRng);
    match generator.generate(12, &charset) {
        Ok(pwd) => println!("Generated: {}", pwd),
        Err(e) => println!("Error: {}", e),
    }
}
