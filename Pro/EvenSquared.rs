use std::fmt::Display;
use std::ops::{Add, Mul, Rem};

#[derive(Debug)]
struct EvenSquareIterator<T> {
    current: T,
    end: T,
    step: T,
}

impl<T> EvenSquareIterator<T> 
where
    T: Copy + PartialOrd + Add<Output = T> + Rem<Output = T> + Mul<Output = T> + From<u8> + PartialEq,
{
    fn new(start: T, end: T) -> Self {
        Self {
            current: start,
            end,
            step: T::from(1),
        }
    }
}

impl<T> Iterator for EvenSquareIterator<T>
where
    T: Copy + PartialOrd + Add<Output = T> + Rem<Output = T> + Mul<Output = T> + From<u8> + PartialEq,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current <= self.end {
            let current = self.current;
            self.current = self.current + self.step;

            if current % T::from(2) == T::from(0) {
                return Some(current * current);
            }
        }
        None
    }
}

trait SumOfSquares<T> {
    fn sum_even_squares(&self, start: T, end: T) -> T;
}

impl<T> SumOfSquares<T> for T
where
    T: Copy + PartialOrd + Add<Output = T> + Rem<Output = T> + Mul<Output = T> + From<u8> + PartialEq + Display,
{
    fn sum_even_squares(&self, start: T, end: T) -> T {
        EvenSquareIterator::new(start, end)
            .inspect(|&square| {

                let original = (1..)
                    .find(|&n| n * n == square && n % 2 == 0)
                    .unwrap_or_else(|| panic!("Invalid square: {}", square));
                println!("Even number: {}, Square: {}", original, square);
            })
            .fold(T::from(0), Add::add)
    }
}

fn pro_version() {
    let calculator = 42i32; 
    let sum = calculator.sum_even_squares(1, 10);

    println!("Sum of squares of even numbers: {}", sum);
}

fn main() {
    pro_version();
}
