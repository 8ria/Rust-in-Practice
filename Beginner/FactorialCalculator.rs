fn factorial_beginner(n: u32) -> u64 {
    let mut result = 1;
    let mut i = 1;
    
    while i <= n {
        result = result * i as u64;
        i = i + 1;
    }
    result
}

fn main() {
    println!("=== BEGINNER VERSION ===");
    let number = 5;
    let result = factorial_beginner(number);
    println!("Factorial of {} is {}", number, result);
}
