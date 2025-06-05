use rand::seq::SliceRandom;

#[derive(Debug)]
enum PasswordError {
    InvalidLength,
}

struct PasswordConfig {
    length: usize,
    include_uppercase: bool,
    include_lowercase: bool,
    include_numbers: bool,
    include_symbols: bool,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            length: 12,
            include_uppercase: true,
            include_lowercase: true,
            include_numbers: true,
            include_symbols: true,
        }
    }
}

fn intermediate_password_generator(config: &PasswordConfig) -> Result<String, PasswordError> {
    if config.length == 0 {
        return Err(PasswordError::InvalidLength);
    }
    
    let mut charset: Vec<u8> = Vec::new();
    
    if config.include_uppercase {
        charset.extend(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if config.include_lowercase {
        charset.extend(b"abcdefghijklmnopqrstuvwxyz");
    }
    if config.include_numbers {
        charset.extend(b"0123456789");
    }
    if config.include_symbols {
        charset.extend(b"!@#$%^&*");
    }
    
    if charset.is_empty() {
        return Err(PasswordError::InvalidLength);
    }
    
    let mut rng = rand::thread_rng();
    let password: String = (0..config.length)
        .map(|_| *charset.choose(&mut rng).unwrap() as char)
        .collect();
    Ok(password)
}

fn main() {
    println!("=== Intermediate Implementation ===");
    let config = PasswordConfig::default();
    match intermediate_password_generator(&config) {
        Ok(pwd) => println!("Generated: {}", pwd),
        Err(e) => println!("Error: {:?}", e),
    }
}
