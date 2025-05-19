fn main() {
    let total_money: f32 = 91.56;
    let total_cents: i32 = (total_money * 100.0).round() as i32;
    let mut remaining_cents = total_cents;

    let ten_dollar_bills = remaining_cents / 1000;  
    remaining_cents = remaining_cents % 1000;       

    let five_dollar_bills = remaining_cents / 500;  
    remaining_cents = remaining_cents % 500;        

    let two_dollar_bills = remaining_cents / 200;   
    remaining_cents = remaining_cents % 200;        

    let one_dollar_bills = remaining_cents / 100;   
    remaining_cents = remaining_cents % 100;        

    println!("tens: {}", ten_dollar_bills);
    println!("fives: {}", five_dollar_bills);
    println!("twos: {}", two_dollar_bills);
    println!("ones: {}", one_dollar_bills);

    if remaining_cents > 0 {
        println!("cents: {:.2}", remaining_cents as f32 / 100.0);
    } else {
        println!("cents: 0.00");
    }
}
