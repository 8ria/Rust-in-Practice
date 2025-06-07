use std::time::Instant;
use std::io::{self, Write};

fn beginner_stopwatch() {
    println!("=== Beginner Stopwatch ===");
    println!("Commands: start, stop, reset, lap, quit");
    
    let mut start_time: Option<Instant> = None;
    let mut total_time = std::time::Duration::new(0, 0);
    let mut is_running = false;
    let mut lap_count = 0;
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim().to_lowercase();
        
        if command == "start" {
            if !is_running {
                start_time = Some(Instant::now());
                is_running = true;
                println!("Stopwatch started!");
            } else {
                println!("Stopwatch is already running!");
            }
        } else if command == "stop" {
            if is_running {
                if let Some(start) = start_time {
                    total_time += start.elapsed();
                }
                is_running = false;
                println!("Stopwatch stopped! Total time: {:.2}s", total_time.as_secs_f64());
            } else {
                println!("Stopwatch is not running!");
            }
        } else if command == "reset" {
            start_time = None;
            total_time = std::time::Duration::new(0, 0);
            is_running = false;
            lap_count = 0;
            println!("Stopwatch reset!");
        } else if command == "lap" {
            if is_running {
                if let Some(start) = start_time {
                    lap_count += 1;
                    let current_total = total_time + start.elapsed();
                    println!("Lap {}: {:.2}s", lap_count, current_total.as_secs_f64());
                }
            } else {
                println!("Stopwatch is not running!");
            }
        } else if command == "quit" {
            println!("Goodbye!");
            break;
        } else {
            println!("Unknown command: {}", command);
        }
    }
}

fn main() {
    beginner_stopwatch();
}
