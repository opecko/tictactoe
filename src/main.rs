use std::io::{self, stdout};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

macro_rules! color_print {
    ($r:expr, $g:expr, $b:expr, $($arg:tt)*) => {
        print!("\x1b[38;2;{};{};{}m{}\x1b[0m", $r, $g, $b, format!($($arg)*));
    };
}

macro_rules! color_println {
    ($r:expr, $g:expr, $b:expr, $($arg:tt)*) => {
        println!("\x1b[38;2;{};{};{}m{}\x1b[0m", $r, $g, $b, format!($($arg)*));
    };
}

fn check_for_win(board: [char; 9]) -> Option<char> {
    // 8 kombinací win
    let wins = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for combo in wins {
        let [a, b, c] = combo;
        if board[a] != ' ' && board[a] == board[b] && board[b] == board[c] {
            return Some(board[a]);
        }
    }
    if !board.contains(&' ') {
        return Some('r');
    }

    None
}
fn print_board(board: [char; 9]) {
    // print 3 rows of 3
    for row in 0..3 {
        for col in 0..3 {
            let idx = row * 3 + col;
            let c = board[idx];
            // if space show 1-based index
            if c == ' ' {
                // print number without newline
                color_print!(87, 87, 87, " {} ", idx + 1);
            } else {
                if c == 'X' {
                    color_print!(255, 85, 70, " X ");
                } else {
                    color_print!(7, 183, 247, " O ");
                }
            }
            if col < 2 {
                print!("|");
            }
        }
        println!();
        if row < 2 {
            println!("-----------");
        }
    }
}

// BOT FUNKCE

fn evaluate(board: [char; 9]) -> Option<i32> {
    match check_for_win(board) {
        Some('O') => Some(10),
        Some('X') => Some(-10),
        Some('r') => Some(0),
        _ => None,
    }
}

fn minimax(board: [char; 9], is_maximizing: bool) -> i32 {
    if let Some(score) = evaluate(board) {
        return score;
    }

    if is_maximizing {
        let mut best = i32::MIN;
        for i in 0..9 {
            if board[i] == ' ' {
                let mut b = board;
                b[i] = 'O';
                best = best.max(minimax(b, false));
            }
        }
        best
    } else {
        let mut best = i32::MAX;
        for i in 0..9 {
            if board[i] == ' ' {
                let mut b = board;
                b[i] = 'X';
                best = best.min(minimax(b, true));
            }
        }
        best
    }
}

fn bot_move(board: [char; 9]) -> usize {
    let mut best_score = i32::MIN;
    let mut best_idx = 0;
    for i in 0..9 {
        if board[i] == ' ' {
            let mut b = board;
            b[i] = 'O';
            let score = minimax(b, false);
            if score > best_score {
                best_score = score;
                best_idx = i;
            }
        }
    }
    best_idx
}

fn main() {
    let mut board = [' '; 9];
    let mut input = String::new();
    let mut current_player: u8 = 0; // 0 => x ; 1 => y
    let winner: char;

    loop {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        print_board(board);
        io::stdin().read_line(&mut input).expect("Chyba na stdin!");

        match input.trim() {
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                let idx = input.trim().parse::<usize>().unwrap() - 1;
                if board[idx] == ' ' {
                    board[idx] = if current_player == 0 { 'X' } else { 'O' };

                    match check_for_win(board) {
                        Some(w) => {
                            winner = w;
                            execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
                            print_board(board);
                            break;
                        }
                        None => println!(),
                    };
                    current_player ^= 1; // switch player
                } else {
                    println!("pole je uz obsazene!");
                }
            }
            _ => println!("index neexistuje!"),
        };
        input.clear();
    }

    match winner {
        'X' => {
            print!("Konec hry! Vyhrál ");
            color_println!(255, 85, 70, "X");
        }
        'O' => {
            print!("Konec hry! Vyhrál ");
            color_println!(7, 183, 247, "O");
        }
        'r' => {
            color_println!(255, 200, 0, "Remíza!");
        }
        e => {
            panic!("ajajaj! {e}");
        }
    };
}
