fn main() {
    // Initial amount of money to break down (in dollars)
    let money_count: f32 = 91.56;
    
    // Convert dollars to cents and round to handle floating point precision issues
    // We work in cents (integers) to avoid floating point arithmetic errors
    let mut remaining_cents : i32 = (money_count * 100.0).round() as i32;
    
    // Define denominations in cents with their display names
    // 1000 cents = $10, 500 cents = $5, 200 cents = $2, 100 cents = $1
    let denominations : [(i32, &str); 4] = [(1000, "ten"), (500, "five"), (200, "two"), (100, "one")];
    
    // Initialize a vector to store the count of each denomination
    let mut counts = vec![0; denominations.len()];

    // Calculate how many of each denomination we can make
    for (i, &(value, _)) in denominations.iter().enumerate() {
        // Integer division gives us the count of this denomination
        counts[i] = remaining_cents / value;  // e.g 9156 / 1000 = 9 (nine $10 bills)
        
        // Modulo gives us the remainder after taking out this denomination
        remaining_cents %= value;  // e.g 9156 % 1000 = 156 (156 cents left)
    }

    // Output the result for each denomination
    for (i, &(_, name)) in denominations.iter().enumerate() {
        println!("{}s: {}", name, counts[i]);
    }

    // Handle any remaining cents that couldn't be converted to dollar bills
    if remaining_cents > 0 {
        // Convert back to dollars for display (with 2 decimal places)
        println!("cents: {:.2}", remaining_cents as f32 / 100.0);
    } else {
        // No remaining cents
        println!("cents: 0.00");
    }
}
