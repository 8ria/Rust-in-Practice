fn intermediate_find_primes(limit: u32) -> Vec<u32> {
    if limit < 2 {
        return Vec::new();
    }
    
    let mut primes = vec![2];
    
    for num in (3..=limit).step_by(2) {
        let sqrt_num = (num as f64).sqrt() as u32;
        let is_prime = primes.iter()
            .take_while(|&&p| p <= sqrt_num)
            .all(|&p| num % p != 0);
        
        if is_prime {
            primes.push(num);
        }
    }
    primes
}

fn main() {
    let limit = 30;
    
    println!("Finding primes up to {}\n", limit);
    
    let intermediate_result = intermediate_find_primes(limit);
    println!("Intermediate: {:?}", intermediate_result);
}
