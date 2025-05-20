use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub const fn opposite(self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

impl Default for Cell {
    fn default() -> Self {
        Self::Empty
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, " "),
            Self::Occupied(player) => write!(f, "{}", player),
        }
    }
}

#[derive(Debug)]
pub enum GameError {
    InvalidPosition,
    PositionOccupied,
    GameFinished,
    InputError,
}

pub type GameResult<T> = Result<T, GameError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(u8);

impl Position {
    pub const fn new(pos: u8) -> Option<Self> {
        if pos <= 8 {
            Some(Self(pos))
        } else {
            None
        }
    }

    pub const fn value(self) -> u8 {
        self.0
    }

    pub const fn from_human_input(input: u8) -> Option<Self> {
        if input >= 1 && input <= 9 {
            Some(Self(input - 1))
        } else {
            None
        }
    }
}

impl TryFrom<u8> for Position {
    type Error = GameError;

    fn try_from(value: u8) -> GameResult<Self> {
        Self::from_human_input(value)
            .ok_or(GameError::InvalidPosition)
    }
}

#[derive(Debug, Clone)]
pub struct Board<const N: usize> {
    cells: [Cell; N],
}

impl Board<9> {
    pub const fn new() -> Self {
        Self {
            cells: [Cell::Empty; 9],
        }
    }

    pub fn make_move(&mut self, position: Position, player: Player) -> GameResult<()> {
        let idx = position.value() as usize;
        match self.cells[idx] {
            Cell::Empty => {
                self.cells[idx] = Cell::Occupied(player);
                Ok(())
            }
            Cell::Occupied(_) => Err(GameError::PositionOccupied),
        }
    }

    pub fn check_winner(&self) -> Option<Player> {
        const WINNING_PATTERNS: [[usize; 3]; 8] = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8],
            [0, 3, 6], [1, 4, 7], [2, 5, 8],
            [0, 4, 8], [2, 4, 6],
        ];

        WINNING_PATTERNS
            .iter()
            .find_map(|&pattern| {
                let cells: Vec<_> = pattern.iter().map(|&i| self.cells[i]).collect();
                match &cells[..] {
                    &[Cell::Occupied(p1), Cell::Occupied(p2), Cell::Occupied(p3)]
                        if p1 == p2 && p2 == p3 => Some(p1),
                    _ => None,
                }
            })
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|&cell| cell != Cell::Empty)
    }
}

impl fmt::Display for Board<9> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

#[derive(Debug, Clone)]
pub enum GameState {
    InProgress { current_player: Player },
    Finished { outcome: GameOutcome },
}

#[derive(Debug, Clone)]
pub enum GameOutcome {
    Winner(Player),
    Draw,
}

impl fmt::Display for GameOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Winner(player) => write!(f, "Player {} wins!", player),
            Self::Draw => write!(f, "It's a tie!"),
        }
    }
}

pub trait PlayerStrategy: fmt::Debug {
    fn get_move(&mut self, board: &Board<9>, player: Player) -> GameResult<Position>;
    fn name(&self) -> &str;
}

#[derive(Debug)]
pub struct HumanPlayer;

impl PlayerStrategy for HumanPlayer {
    fn get_move(&mut self, _board: &Board<9>, player: Player) -> GameResult<Position> {
        println!("Player {}, enter position (1-9):", player);
        io::stdout().flush().map_err(|_| GameError::InputError)?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|_| GameError::InputError)?;

        let position_num: u8 = input
            .trim()
            .parse()
            .map_err(|_| GameError::InvalidPosition)?;

        Position::try_from(position_num)
    }

    fn name(&self) -> &str {
        "Human"
    }
}

pub struct Game {
    board: Board<9>,
    state: GameState,
    players: HashMap<Player, Box<dyn PlayerStrategy>>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            state: GameState::InProgress {
                current_player: Player::X,
            },
            players: HashMap::new(),
        }
    }

    pub fn set_player_strategy(&mut self, player: Player, strategy: Box<dyn PlayerStrategy>) {
        self.players.insert(player, strategy);
    }

    pub fn play_game(&mut self) -> GameResult<GameOutcome> {

        if self.players.is_empty() {
            self.set_player_strategy(Player::X, Box::new(HumanPlayer));
            self.set_player_strategy(Player::O, Box::new(HumanPlayer));
        }

        while let GameState::InProgress { current_player } = self.state {
            print!("{}", self.board);

            if let Some(strategy) = self.players.get_mut(&current_player) {
                match strategy.get_move(&self.board, current_player) {
                    Ok(position) => {
                        if let Err(e) = self.make_move(position, current_player) {
                            match e {
                                GameError::InvalidPosition => println!("Invalid input! Enter 1-9"),
                                GameError::PositionOccupied => println!("Position taken! Try again"),
                                _ => println!("Error occurred"),
                            }
                            continue;
                        }
                    }
                    Err(e) => {
                        match e {
                            GameError::InvalidPosition => println!("Invalid input! Enter 1-9"),
                            _ => println!("Invalid input! Enter 1-9"),
                        }
                        continue;
                    }
                }
            } else {
                return Err(GameError::GameFinished);
            }
        }

        print!("{}", self.board);

        if let GameState::Finished { outcome } = &self.state {
            println!("{}", outcome);
            Ok(outcome.clone())
        } else {
            unreachable!("Game should be finished")
        }
    }

    fn make_move(&mut self, position: Position, player: Player) -> GameResult<()> {
        match &self.state {
            GameState::InProgress { current_player } if *current_player == player => {
                self.board.make_move(position, player)?;

                self.state = if let Some(winner) = self.board.check_winner() {
                    GameState::Finished {
                        outcome: GameOutcome::Winner(winner),
                    }
                } else if self.board.is_full() {
                    GameState::Finished {
                        outcome: GameOutcome::Draw,
                    }
                } else {
                    GameState::InProgress {
                        current_player: player.opposite(),
                    }
                };
                Ok(())
            }
            GameState::Finished { .. } => Err(GameError::GameFinished),
            _ => Err(GameError::InvalidPosition),
        }
    }
}

fn main() -> io::Result<()> {
    let mut game = Game::new();
    let _ = game.play_game();
    Ok(())
}
