fn main() {
    let money_count: f32 = 91.56;
    let mut remaining_cents : i32 = (money_count * 100.0).round() as i32;
    let denominations : [(i32, &str); 4] = [(1000, "ten"), (500, "five"), (200, "two"), (100, "one")];
    let mut counts = vec![0; denominations.len()];

    for (i, &(value, _)) in denominations.iter().enumerate() {
        counts[i] = remaining_cents / value;  
        remaining_cents %= value;  
    }

    for (i, &(_, name)) in denominations.iter().enumerate() {
        println!("{}s: {}", name, counts[i]);
    }

    if remaining_cents > 0 {
        println!("cents: {:.2}", remaining_cents as f32 / 100.0);
    } else {
        println!("cents: 0.00");
    }
}
