fn beginner_version() {
    let mut sum = 0;
    let start = 1;
    let end = 10;

    for i in start..=end {
        if i % 2 == 0 {
            let square = i * i;
            sum = sum + square;
            println!("Even number: {}, Square: {}", i, square);
        }
    }
    println!("Sum of squares of even numbers: {}", sum);
}

fn main() {
    beginner_version();
}
