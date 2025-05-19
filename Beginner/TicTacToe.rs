use std::io;

fn main() {
    let mut board = [' '; 9];
    let mut current_player = 'X';

    loop {
        show_board(&board);
        println!("Player {}, enter position (1-9):", current_player);

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read input");

        let position = match user_input.trim().parse::<usize>() {
            Ok(num) => {
                if num >= 1 && num <= 9 {
                    num - 1  
                } else {
                    println!("Invalid input! Enter 1-9");
                    continue;
                }
            }
            Err(_) => {
                println!("Invalid input! Enter 1-9");
                continue;
            }
        };

        if board[position] != ' ' {
            println!("Position taken! Try again");
            continue;
        }

        board[position] = current_player;

        if player_wins(&board, current_player) {
            show_board(&board);
            println!("Player {} wins!", current_player);
            break;
        }

        if is_board_full(&board) {
            show_board(&board);
            println!("It's a tie!");
            break;
        }

        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}

fn show_board(board: &[char; 9]) {
    println!();
    println!(" {} | {} | {} ", board[0], board[1], board[2]);
    println!("-----------");
    println!(" {} | {} | {} ", board[3], board[4], board[5]);
    println!("-----------");
    println!(" {} | {} | {} ", board[6], board[7], board[8]);
    println!();
}

fn player_wins(board: &[char; 9], player: char) -> bool {
    let winning_lines = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],

        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],

        [0, 4, 8],
        [2, 4, 6],
    ];

    for line in &winning_lines {
        if board[line[0]] == player && board[line[1]] == player && board[line[2]] == player {
            return true;
        }
    }
    false
}

fn is_board_full(board: &[char; 9]) -> bool {
    for position in board {
        if *position == ' ' {
            return false;
        }
    }
    true
}
