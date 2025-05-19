fn main() {
    // We have $91.56 and want to break it down into bills and coins
    let money_amount = 91.56;
    
    // Convert to cents so we can work with whole numbers
    // $91.56 = 9156 cents (multiply by 100)
    let total_cents = (money_amount * 100.0) as i32;
    
    // Start with all our cents
    let mut cents_left = total_cents;
    
    // Step 1: Count $10 bills
    // How many times does 1000 cents ($10) go into our total?
    let ten_bills = cents_left / 1000;
    // Subtract the cents we used for $10 bills
    cents_left = cents_left - (ten_bills * 1000);
    
    // Step 2: Count $5 bills
    // How many times does 500 cents ($5) go into what's left?
    let five_bills = cents_left / 500;
    // Subtract the cents we used for $5 bills
    cents_left = cents_left - (five_bills * 500);
    
    // Step 3: Count $2 bills  
    // How many times does 200 cents ($2) go into what's left?
    let two_bills = cents_left / 200;
    // Subtract the cents we used for $2 bills
    cents_left = cents_left - (two_bills * 200);
    
    // Step 4: Count $1 bills
    // How many times does 100 cents ($1) go into what's left?
    let one_bills = cents_left / 100;
    // Subtract the cents we used for $1 bills
    cents_left = cents_left - (one_bills * 100);
    
    // Now cents_left has only the coins (less than $1)
    
    // Show our results
    println!("tens: {}", ten_bills);
    println!("fives: {}", five_bills); 
    println!("twos: {}", two_bills);
    println!("ones: {}", one_bills);
    println!("cents: {}", cents_left);
}
