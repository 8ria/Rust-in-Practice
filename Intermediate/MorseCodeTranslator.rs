use std::collections::HashMap;

struct MorseTranslator {
    encode_map: HashMap<char, &'static str>,
    decode_map: HashMap<&'static str, char>,
}

impl MorseTranslator {
    fn new() -> Self {
        let pairs = [
            ('A', ".-"), ('B', "-..."), ('C', "-.-."), ('D', "-.."), ('E', "."),
            ('F', "..-."), ('G', "--."), ('H', "...."), ('I', ".."), ('J', ".---"),
            ('K', "-.-"), ('L', ".-.."), ('M', "--"), ('N', "-."), ('O', "---"),
            ('P', ".--."), ('Q', "--.-"), ('R', ".-."), ('S', "..."), ('T', "-"),
            ('U', "..-"), ('V', "...-"), ('W', ".--"), ('X', "-..-"), ('Y', "-.--"),
            ('Z', "--.."), ('0', "-----"), ('1', ".----"), ('2', "..---"),
            ('3', "...--"), ('4', "....-"), ('5', "....."), ('6', "-...."),
            ('7', "--..."), ('8', "---.."), ('9', "----."),
        ];
        
        let encode_map: HashMap<char, &'static str> = pairs.iter().cloned().collect();
        let decode_map: HashMap<&'static str, char> = pairs.iter().map(|(c, m)| (*m, *c)).collect();
        
        Self { encode_map, decode_map }
    }
    
    fn encode(&self, text: &str) -> Result<String, String> {
        let mut result = Vec::new();
        
        for ch in text.to_uppercase().chars() {
            match ch {
                ' ' => result.push("/".to_string()),
                c if self.encode_map.contains_key(&c) => {
                    result.push(self.encode_map[&c].to_string());
                }
                _ => return Err(format!("Character '{}' not supported", ch)),
            }
        }
        
        Ok(result.join(" "))
    }
    
    fn decode(&self, morse: &str) -> Result<String, String> {
        let mut result = String::new();
        
        for word in morse.split(" / ") {
            if !result.is_empty() {
                result.push(' ');
            }
            
            for code in word.split_whitespace() {
                if let Some(&ch) = self.decode_map.get(code) {
                    result.push(ch);
                } else if !code.is_empty() {
                    return Err(format!("Morse code '{}' not recognized", code));
                }
            }
        }
        Ok(result)
    }
}

fn intermediate_version() {
    println!("\n=== INTERMEDIATE VERSION ===");
    
    let translator = MorseTranslator::new();
    let input = "HELLO WORLD 123";
    
    match translator.encode(input) {
        Ok(morse) => {
            println!("Input: {}", input);
            println!("Morse: {}", morse);
            
            match translator.decode(&morse) {
                Ok(decoded) => println!("Decoded back: {}", decoded),
                Err(e) => println!("Decode error: {}", e),
            }
        }
        Err(e) => println!("Encode error: {}", e),
    }
}

fn main() {
    intermediate_version();
}
