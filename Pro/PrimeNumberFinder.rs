struct PrimeSieve {
    sieve: Vec<bool>,
    limit: usize,
}

impl PrimeSieve {
    fn new(limit: u32) -> Self {
        let limit = limit as usize;
        Self {
            sieve: vec![true; limit + 1],
            limit,
        }
    }
    
    fn sieve_primes(&mut self) -> impl Iterator<Item = u32> + '_ {
        if self.limit < 2 {
            return Box::new(std::iter::empty()) as Box<dyn Iterator<Item = u32>>;
        }
        
        self.sieve[0] = false;
        if self.limit >= 1 { self.sieve[1] = false; }
        
        let sqrt_limit = (self.limit as f64).sqrt() as usize;
        
        for i in 2..=sqrt_limit {
            if self.sieve[i] {
                (i * i..=self.limit)
                    .step_by(i)
                    .for_each(|multiple| self.sieve[multiple] = false);
            }
        }
        
        Box::new(
            self.sieve
                .iter()
                .enumerate()
                .skip(2)
                .filter_map(|(i, &is_prime)| {
                    if is_prime { Some(i as u32) } else { None }
                })
        )
    }
}

fn professional_find_primes(limit: u32) -> Vec<u32> {
    let mut sieve = PrimeSieve::new(limit);
    sieve.sieve_primes().collect()
}

fn main() {
    let limit = 30;
    
    println!("Finding primes up to {}\n", limit);
    
    let professional_result = professional_find_primes(limit);
    println!("Professional: {:?}", professional_result);
}
