use std::time::{Duration, Instant};
use std::io::{self, Write};

#[derive(Debug)]
enum StopwatchCommand {
    Start,
    Stop,
    Reset,
    Lap,
    Status,
    Quit,
}

impl StopwatchCommand {
    fn from_str(s: &str) -> Result<Self, String> {
        match s.trim().to_lowercase().as_str() {
            "start" => Ok(StopwatchCommand::Start),
            "stop" => Ok(StopwatchCommand::Stop),
            "reset" => Ok(StopwatchCommand::Reset),
            "lap" => Ok(StopwatchCommand::Lap),
            "status" => Ok(StopwatchCommand::Status),
            "quit" => Ok(StopwatchCommand::Quit),
            _ => Err(format!("Unknown command: {}", s)),
        }
    }
}

struct Stopwatch {
    start_time: Option<Instant>,
    accumulated_time: Duration,
    is_running: bool,
    lap_count: usize,
}

impl Stopwatch {
    fn new() -> Self {
        Self {
            start_time: None,
            accumulated_time: Duration::new(0, 0),
            is_running: false,
            lap_count: 0,
        }
    }
    
    fn start(&mut self) -> Result<String, String> {
        if self.is_running {
            Err("Stopwatch is already running!".to_string())
        } else {
            self.start_time = Some(Instant::now());
            self.is_running = true;
            Ok("Stopwatch started!".to_string())
        }
    }
    
    fn stop(&mut self) -> Result<String, String> {
        if !self.is_running {
            Err("Stopwatch is not running!".to_string())
        } else {
            if let Some(start) = self.start_time {
                self.accumulated_time += start.elapsed();
            }
            self.is_running = false;
            Ok(format!("Stopwatch stopped! Total time: {:.2}s", self.accumulated_time.as_secs_f64()))
        }
    }
    
    fn reset(&mut self) -> String {
        self.start_time = None;
        self.accumulated_time = Duration::new(0, 0);
        self.is_running = false;
        self.lap_count = 0;
        "Stopwatch reset!".to_string()
    }
    
    fn lap(&mut self) -> Result<String, String> {
        if !self.is_running {
            Err("Stopwatch is not running!".to_string())
        } else {
            self.lap_count += 1;
            let current_time = self.get_current_time();
            Ok(format!("Lap {}: {:.2}s", self.lap_count, current_time.as_secs_f64()))
        }
    }
    
    fn get_current_time(&self) -> Duration {
        let running_time = if let Some(start) = self.start_time {
            start.elapsed()
        } else {
            Duration::new(0, 0)
        };
        self.accumulated_time + running_time
    }
}

fn intermediate_stopwatch() {
    println!("=== Intermediate Stopwatch ===");
    println!("Commands: start, stop, reset, lap, quit");
    
    let mut stopwatch = Stopwatch::new();
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        }
        
        let command = match StopwatchCommand::from_str(&input) {
            Ok(cmd) => cmd,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        
        let result = match command {
            StopwatchCommand::Start => stopwatch.start(),
            StopwatchCommand::Stop => stopwatch.stop(),
            StopwatchCommand::Reset => Ok(stopwatch.reset()),
            StopwatchCommand::Lap => stopwatch.lap(),
            StopwatchCommand::Status => {
                let time = stopwatch.get_current_time();
                Ok(format!("Current time: {:.2}s ({})", 
                    time.as_secs_f64(), 
                    if stopwatch.is_running { "running" } else { "stopped" }))
            },
            StopwatchCommand::Quit => {
                println!("Goodbye!");
                break;
            }
        };
        
        match result {
            Ok(msg) => println!("{}", msg),
            Err(err) => println!("{}", err),
        }
    }
}

fn main() {
    intermediate_stopwatch();
}
