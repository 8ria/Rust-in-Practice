use std::fmt;

#[derive(Debug)]
enum FactorialError {
    Overflow,
}

impl fmt::Display for FactorialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FactorialError::Overflow => write!(f, "Result would overflow"),
        }
    }
}

fn factorial_intermediate(n: u32) -> Result<u64, FactorialError> {
    if n > 20 {
        return Err(FactorialError::Overflow);
    }
    
    let result = (1..=n)
        .map(|x| x as u64)
        .try_fold(1u64, |acc, x| acc.checked_mul(x))
        .ok_or(FactorialError::Overflow)?;
    
    Ok(result)
}

fn main() {
    println!("\n=== INTERMEDIATE VERSION ===");
  
    let numbers = vec![5, 10, 21];
    for num in numbers {
        match factorial_intermediate(num) {
            Ok(result) => println!("Factorial of {} is {}", num, result),
            Err(e) => println!("Error calculating factorial of {}: {}", num, e),
        }
    }
}
