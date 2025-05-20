use std::fmt;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

impl Player {
    fn other(self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Occupied(Player),
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::Occupied(player) => write!(f, "{}", player),
        }
    }
}

#[derive(Debug)]
enum GameError {
    InvalidInput,
    PositionOccupied,
}

#[derive(Debug)]
struct Board {
    cells: [Cell; 9],
}

impl Board {
    fn new() -> Self {
        Self {
            cells: [Cell::Empty; 9],
        }
    }

    fn make_move(&mut self, position: usize, player: Player) -> Result<(), GameError> {
        if position >= 9 {
            return Err(GameError::InvalidInput);
        }

        match self.cells[position] {
            Cell::Empty => {
                self.cells[position] = Cell::Occupied(player);
                Ok(())
            }
            Cell::Occupied(_) => Err(GameError::PositionOccupied),
        }
    }

    fn check_winner(&self) -> Option<Player> {
        const WINNING_PATTERNS: [[usize; 3]; 8] = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8],
            [0, 3, 6], [1, 4, 7], [2, 5, 8],
            [0, 4, 8], [2, 4, 6],
        ];

        for pattern in &WINNING_PATTERNS {
            match (self.cells[pattern[0]], self.cells[pattern[1]], self.cells[pattern[2]]) {
                (Cell::Occupied(p1), Cell::Occupied(p2), Cell::Occupied(p3)) 
                    if p1 == p2 && p2 == p3 => return Some(p1),
                _ => continue,
            }
        }
        None
    }

    fn is_full(&self) -> bool {
        self.cells.iter().all(|&cell| cell != Cell::Empty)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, " {} | {} | {} ", 
                 self.cells[0], self.cells[1], self.cells[2])?;
        writeln!(f, "-----------")?;
        writeln!(f, " {} | {} | {} ", 
                 self.cells[3], self.cells[4], self.cells[5])?;
        writeln!(f, "-----------")?;
        writeln!(f, " {} | {} | {} ", 
                 self.cells[6], self.cells[7], self.cells[8])?;
        writeln!(f)
    }
}

#[derive(Debug)]
enum GameState {
    Playing(Player),
    Won(Player),
    Draw,
}

struct Game {
    board: Board,
    state: GameState,
}

impl Game {
    fn new() -> Self {
        Self {
            board: Board::new(),
            state: GameState::Playing(Player::X),
        }
    }

    fn make_move(&mut self, position: usize) -> Result<(), GameError> {
        match &self.state {
            GameState::Playing(current_player) => {
                let player = *current_player;
                self.board.make_move(position, player)?;

                self.state = if let Some(winner) = self.board.check_winner() {
                    GameState::Won(winner)
                } else if self.board.is_full() {
                    GameState::Draw
                } else {
                    GameState::Playing(player.other())
                };
                Ok(())
            }
            _ => Err(GameError::InvalidInput),
        }
    }

    fn current_player(&self) -> Option<Player> {
        match self.state {
            GameState::Playing(player) => Some(player),
            _ => None,
        }
    }

    fn is_over(&self) -> bool {
        !matches!(self.state, GameState::Playing(_))
    }
}

fn get_user_input() -> Result<usize, GameError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map_err(|_| GameError::InvalidInput)?;

    let position = input.trim().parse::<usize>()
        .map_err(|_| GameError::InvalidInput)?;

    if position >= 1 && position <= 9 {
        Ok(position - 1)
    } else {
        Err(GameError::InvalidInput)
    }
}

fn main() -> io::Result<()> {
    let mut game = Game::new();

    while !game.is_over() {
        print!("{}", game.board);

        if let Some(player) = game.current_player() {
            println!("Player {}, enter position (1-9):", player);

            match get_user_input() {
                Ok(position) => {
                    if let Err(error) = game.make_move(position) {
                        match error {
                            GameError::InvalidInput => println!("Invalid input! Enter 1-9"),
                            GameError::PositionOccupied => println!("Position taken! Try again"),
                        }
                    }
                }
                Err(_) => {
                    println!("Invalid input! Enter 1-9");
                }
            }
        }
    }

    print!("{}", game.board);
    match game.state {
        GameState::Won(player) => println!("Player {} wins!", player),
        GameState::Draw => println!("It's a tie!"),
        _ => unreachable!(),
    }
    Ok(())
}
