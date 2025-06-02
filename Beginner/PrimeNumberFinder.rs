fn beginner_find_primes(limit: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    
    for num in 2..=limit {
        let mut is_prime = true;
        
        for i in 2..num {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }
        
        if is_prime {
            primes.push(num);
        }
    }
    primes
}

fn main() {
    let limit = 30;
    
    println!("Finding primes up to {}\n", limit);
    
    let beginner_result = beginner_find_primes(limit);
    println!("Beginner: {:?}", beginner_result);
}
