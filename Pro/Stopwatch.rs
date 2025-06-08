use std::time::{Duration, Instant};
use std::io::{self, Write};
use std::fmt;
use std::sync::{Arc, Mutex};

trait TimeProvider: Send + Sync {
    fn now(&self) -> Instant;
}

struct SystemTimeProvider;

impl TimeProvider for SystemTimeProvider {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Command {
    Start,
    Stop,
    Reset,
    Lap,
    Status,
    Quit,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Start => write!(f, "start"),
            Command::Stop => write!(f, "stop"),
            Command::Reset => write!(f, "reset"),
            Command::Lap => write!(f, "lap"),
            Command::Status => write!(f, "status"),
            Command::Quit => write!(f, "quit"),
        }
    }
}

#[derive(Debug)]
enum ParseError {
    UnknownCommand(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnknownCommand(cmd) => write!(f, "Unknown command: {}", cmd),
        }
    }
}

impl std::str::FromStr for Command {
    type Err = ParseError;
    
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "start" => Ok(Command::Start),
            "stop" => Ok(Command::Stop),
            "reset" => Ok(Command::Reset),
            "lap" => Ok(Command::Lap),
            "status" => Ok(Command::Status),
            "quit" => Ok(Command::Quit),
            _ => Err(ParseError::UnknownCommand(s.to_string())),
        }
    }
}

#[derive(Debug)]
enum StopwatchError {
    AlreadyRunning,
    NotRunning,
}

impl fmt::Display for StopwatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StopwatchError::AlreadyRunning => write!(f, "Stopwatch is already running!"),
            StopwatchError::NotRunning => write!(f, "Stopwatch is not running!"),
        }
    }
}

impl std::error::Error for StopwatchError {}

type Result<T> = std::result::Result<T, StopwatchError>;

#[derive(Debug, Clone)]
struct LapTime {
    lap_number: usize,
    total_time: Duration,
}

impl fmt::Display for LapTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lap {}: {:.2}s", self.lap_number, self.total_time.as_secs_f64())
    }
}

struct AdvancedStopwatch<T: TimeProvider> {
    time_provider: Arc<T>,
    state: Arc<Mutex<StopwatchState>>,
}

#[derive(Debug)]
struct StopwatchState {
    start_time: Option<Instant>,
    accumulated_time: Duration,
    is_running: bool,
    lap_times: Vec<LapTime>,
}

impl<T: TimeProvider> AdvancedStopwatch<T> {
    fn new(time_provider: T) -> Self {
        Self {
            time_provider: Arc::new(time_provider),
            state: Arc::new(Mutex::new(StopwatchState {
                start_time: None,
                accumulated_time: Duration::ZERO,
                is_running: false,
                lap_times: Vec::new(),
            })),
        }
    }
    
    fn execute_command(&self, command: Command) -> std::result::Result<String, Box<dyn std::error::Error>> {
        match command {
            Command::Start => self.start().map_err(Into::into),
            Command::Stop => self.stop().map_err(Into::into),
            Command::Reset => Ok(self.reset()),
            Command::Lap => self.lap().map_err(Into::into),
            Command::Status => Ok(self.status()),
            Command::Quit => Ok("Goodbye!".to_string()),
        }
    }
    
    fn start(&self) -> Result<String> {
        let mut state = self.state.lock().unwrap();
        if state.is_running {
            Err(StopwatchError::AlreadyRunning)
        } else {
            state.start_time = Some(self.time_provider.now());
            state.is_running = true;
            Ok("Stopwatch started!".to_string())
        }
    }
    
    fn stop(&self) -> Result<String> {
        let mut state = self.state.lock().unwrap();
        if !state.is_running {
            Err(StopwatchError::NotRunning)
        } else {
            if let Some(start) = state.start_time {
                state.accumulated_time += start.elapsed();
            }
            state.is_running = false;
            Ok(format!("Stopwatch stopped! Total time: {:.2}s", 
                state.accumulated_time.as_secs_f64()))
        }
    }
    
    fn reset(&self) -> String {
        let mut state = self.state.lock().unwrap();
        *state = StopwatchState {
            start_time: None,
            accumulated_time: Duration::ZERO,
            is_running: false,
            lap_times: Vec::new(),
        };
        "Stopwatch reset!".to_string()
    }
    
    fn lap(&self) -> Result<String> {
        let mut state = self.state.lock().unwrap();
        if !state.is_running {
            Err(StopwatchError::NotRunning)
        } else {
            let current_time = self.get_current_time_internal(&state);
            let lap_number = state.lap_times.len() + 1;
            let lap_time = LapTime {
                lap_number,
                total_time: current_time,
            };
            state.lap_times.push(lap_time.clone());
            Ok(lap_time.to_string())
        }
    }
    
    fn status(&self) -> String {
        let state = self.state.lock().unwrap();
        let current_time = self.get_current_time_internal(&state);
        format!("Current time: {:.2}s ({})", 
            current_time.as_secs_f64(),
            if state.is_running { "running" } else { "stopped" })
    }
    
    fn get_current_time_internal(&self, state: &StopwatchState) -> Duration {
        let running_time = if let Some(start) = state.start_time {
            start.elapsed()
        } else {
            Duration::ZERO
        };
        state.accumulated_time + running_time
    }
}

fn get_user_input() -> io::Result<String> {
    print!("> ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn pro_stopwatch() {
    println!("=== Pro Stopwatch ===");
    println!("Commands: start, stop, reset, lap, quit");
    
    let stopwatch = AdvancedStopwatch::new(SystemTimeProvider);
    
    loop {
        let input = match get_user_input() {
            Ok(input) => input,
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        };
        
        let command: Command = match input.parse() {
            Ok(cmd) => cmd,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        
        if command == Command::Quit {
            println!("Goodbye!");
            break;
        }
        
        match stopwatch.execute_command(command) {
            Ok(message) => println!("{}", message),
            Err(e) => println!("{}", e),
        }
    }
}

fn main() {
    pro_stopwatch();
}
