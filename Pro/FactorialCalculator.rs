trait Factorial {
    type Output;
    type Error;

    fn factorial(self) -> Result<Self::Output, Self::Error>;
}

impl Factorial for u32 {
    type Output = u64;
    type Error = &'static str;

    fn factorial(self) -> Result<Self::Output, Self::Error> {
        if self > 20 {
            return Err("Input too large for safe calculation");
        }
        Ok((1..=self as u64).product())
    }
}

impl Factorial for u64 {
    type Output = u64;
    type Error = &'static str;

    fn factorial(self) -> Result<Self::Output, Self::Error> {
        if self > 20 {
            return Err("Input too large for safe calculation");
        }

        Ok((1..=self).product())
    }
}

const fn const_factorial(n: u32) -> u64 {
    if n <= 1 {
        1
    } else {
        n as u64 * const_factorial(n - 1)
    }
}

#[derive(Default)]
struct FactorialCalculator {
    check_overflow: bool,
    max_input: Option<u32>,
}

impl FactorialCalculator {
    fn new() -> Self {
        Self::default()
    }

    fn with_overflow_check(mut self, check: bool) -> Self {
        self.check_overflow = check;
        self
    }

    fn with_max_input(mut self, max: u32) -> Self {
        self.max_input = Some(max);
        self
    }

    fn calculate(&self, n: u32) -> Result<u64, Box<dyn std::error::Error>> {
        if let Some(max) = self.max_input {
            if n > max {
                return Err(format!("Input {} exceeds maximum {}", n, max).into());
            }
        }

        if self.check_overflow && n > 20 {
            return Err("Potential overflow detected".into());
        }

        Ok((1..=n as u64).product())
    }
}

macro_rules! factorial_fn {
    ($name:ident, $n:expr) => {
        const fn $name() -> u64 {
            const_factorial($n)
        }
    };
}

factorial_fn!(factorial_5, 5);
factorial_fn!(factorial_10, 10);

fn main() {
    println!("\n=== PRO VERSION ===");

    println!("Trait-based: {}", 5u32.factorial().unwrap_or(0));
    println!("Compile-time factorial of 5: {}", factorial_5());
    println!("Compile-time factorial of 10: {}", factorial_10());

    let calculator = FactorialCalculator::new()
        .with_overflow_check(true)
        .with_max_input(15);

    match calculator.calculate(12) {
        Ok(result) => println!("Builder pattern result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    let factorial_closure = |n: u32| -> u64 {
        (1..=n as u64).product()
    };

    let numbers = [1, 5, 8, 12];
    let results: Vec<_> = numbers
        .iter()
        .map(|&n| (n, factorial_closure(n)))
        .collect();

    println!("Functional results: {:?}", results);
}
