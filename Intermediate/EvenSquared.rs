fn intermediate_version() {
    let range = 1..=10;

    let even_squares: Vec<i32> = range
        .filter(|&n| n % 2 == 0)  
        .map(|n| {                
            let square = n * n;
            println!("Even number: {}, Square: {}", n, square);
            square
        })
        .collect();               

    let sum: i32 = even_squares.iter().sum();
    println!("Sum of squares of even numbers: {}", sum);
}

fn main() {
    intermediate_version();
}
