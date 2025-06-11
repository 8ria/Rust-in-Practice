use std::collections::HashMap;

fn beginner_version() {
    println!("=== BEGINNER VERSION ===");
    let mut morse_map = HashMap::new();
    morse_map.insert('A', ".-");
    morse_map.insert('B', "-...");
    morse_map.insert('C', "-.-.");
    morse_map.insert('D', "-..");
    morse_map.insert('E', ".");
    morse_map.insert('F', "..-.");
    morse_map.insert('G', "--.");
    morse_map.insert('H', "....");
    morse_map.insert('I', "..");
    morse_map.insert('J', ".---");
    morse_map.insert('K', "-.-");
    morse_map.insert('L', ".-..");
    morse_map.insert('M', "--");
    morse_map.insert('N', "-.");
    morse_map.insert('O', "---");
    morse_map.insert('P', ".--.");
    morse_map.insert('Q', "--.-");
    morse_map.insert('R', ".-.");
    morse_map.insert('S', "...");
    morse_map.insert('T', "-");
    morse_map.insert('U', "..-");
    morse_map.insert('V', "...-");
    morse_map.insert('W', ".--");
    morse_map.insert('X', "-..-");
    morse_map.insert('Y', "-.--");
    morse_map.insert('Z', "--..");
    morse_map.insert('0', "-----");
    morse_map.insert('1', ".----");
    morse_map.insert('2', "..---");
    morse_map.insert('3', "...--");
    morse_map.insert('4', "....-");
    morse_map.insert('5', ".....");
    morse_map.insert('6', "-....");
    morse_map.insert('7', "--...");
    morse_map.insert('8', "---..");
    morse_map.insert('9', "----.");
    
    let input = "HELLO WORLD 123";
    let mut result = String::new();
    
    for ch in input.chars() {
        if ch == ' ' {
            result.push_str("/ ");
        } else if let Some(morse) = morse_map.get(&ch) {
            result.push_str(morse);
            result.push(' ');
        }
    }
    
    println!("Input: {}", input);
    println!("Morse: {}", result.trim());
}

fn main() {
    beginner_version();
}
