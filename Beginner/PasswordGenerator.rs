use rand::Rng;

fn beginner_password_generator(length: usize) -> String {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*";
    let mut password = String::new();
    let mut rng = rand::thread_rng();
    
    for _ in 0..length {
        let index = rng.gen_range(0..chars.len());
        let char = chars.chars().nth(index).unwrap();
        password.push(char);
    }
    password
}

fn main() {
    println!("=== Beginner Implementation ===");
    let beginner_pwd = beginner_password_generator(12);
    println!("Generated: {}", beginner_pwd);
}
