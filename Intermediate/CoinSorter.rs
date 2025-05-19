fn main() {
    // The amount of money we want to break down into bills and coins
    let total_money: f32 = 91.56;
    
    // Convert dollars to cents to avoid floating point issues
    // Example: $91.56 becomes 9156 cents
    let total_cents: i32 = (total_money * 100.0).round() as i32;
    let mut remaining_cents = total_cents;
    
    // Calculate how many $10 bills we can make
    let ten_dollar_bills = remaining_cents / 1000;  // 1000 cents = $10
    remaining_cents = remaining_cents % 1000;       // Get the leftover cents
    
    // Calculate how many $5 bills we can make from what's left
    let five_dollar_bills = remaining_cents / 500;  // 500 cents = $5
    remaining_cents = remaining_cents % 500;        // Get the leftover cents
    
    // Calculate how many $2 bills we can make from what's left
    let two_dollar_bills = remaining_cents / 200;   // 200 cents = $2
    remaining_cents = remaining_cents % 200;        // Get the leftover cents
    
    // Calculate how many $1 bills we can make from what's left
    let one_dollar_bills = remaining_cents / 100;   // 100 cents = $1
    remaining_cents = remaining_cents % 100;        // Get the leftover cents
    
    // Print out how many of each bill we have
    println!("tens: {}", ten_dollar_bills);
    println!("fives: {}", five_dollar_bills);
    println!("twos: {}", two_dollar_bills);
    println!("ones: {}", one_dollar_bills);
    
    // Print the remaining cents (less than $1)
    if remaining_cents > 0 {
        println!("cents: {:.2}", remaining_cents as f32 / 100.0);
    } else {
        println!("cents: 0.00");
    }
}
