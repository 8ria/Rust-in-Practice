fn main() {
    let money_amount = 91.56;
    let total_cents = (money_amount * 100.0) as i32;
    let mut cents_left = total_cents;
    
    let ten_bills = cents_left / 1000;
    cents_left = cents_left - (ten_bills * 1000);

    let five_bills = cents_left / 500;
    cents_left = cents_left - (five_bills * 500);

    let two_bills = cents_left / 200;
    cents_left = cents_left - (two_bills * 200);

    let one_bills = cents_left / 100;
    cents_left = cents_left - (one_bills * 100);

    println!("tens: {}", ten_bills);
    println!("fives: {}", five_bills); 
    println!("twos: {}", two_bills);
    println!("ones: {}", one_bills);
    println!("cents: {}", cents_left);
}
