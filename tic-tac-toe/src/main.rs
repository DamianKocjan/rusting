use std::io;

use rand::thread_rng;
use rand::Rng;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn to_number(number: &String) -> i8 {
    number.trim().parse().expect("Not a number!")
}

#[derive(PartialEq)]
enum Player {
    None = 0,
    X = 1,
    O = 2,
}

type Board = Vec<Player>;

fn create_board() -> Board {
    let mut board: Board = vec![];

    for _ in 0..9 {
        board.push(Player::None);
    }

    board
}

fn insert_symbol_to_board(x: usize, p: Player, board: &mut Board) {
    if board[x] != Player::None {
        return;
    }

    match p {
        Player::X => {
            board[x] = p;
        }
        _ => {
            board[x] = p;
        }
    }
}

#[derive(PartialEq)]
enum GameStatus {
    Draw,
    X,
    O,
    None,
}

const COMBINATIONS: [[usize; 3]; 9] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [6, 4, 2],
    [0, 4, 8],
    [2, 4, 6],
    [1, 4, 7],
    [2, 5, 8],
];

fn check_board_status(board: &Board) -> GameStatus {
    for combination in COMBINATIONS {
        let a = combination[0];
        let b = combination[1];
        let c = combination[2];

        if board[a] == board[b] && board[b] == board[c] {
            match board[a] {
                Player::X => return GameStatus::X,
                Player::O => return GameStatus::O,
                _ => {}
            }
        }
    }

    let mut i = 0;
    for x in board {
        if x != &Player::None {
            i += 1;
        }
    }

    if i == 9 {
        return GameStatus::Draw;
    }

    return GameStatus::None;
}

fn starting_player() -> &'static Player {
    println!("Who should start? \n 1) X\n 2) O\n 3) Random");

    let input = to_number(&get_input());

    match input {
        1 => return &Player::X,
        2 => return &Player::O,
        _ => {
            let mut rng = thread_rng();
            let random: u32 = rng.gen_range(0..1);

            if random == 0 {
                return &Player::X;
            }
            return &Player::O;
        }
    }
}

fn print_board(board: &Board) {
    let mut i = 0;
    for el in board {
        match el {
            Player::X => print!("X"),
            Player::O => print!("O"),
            _ => print!(" "),
        }

        i += 1;

        if i % 3 == 0 {
            print!("\n");
        }
    }
}

fn game() {
    let mut board: Board = create_board();

    let mut whose_turn = starting_player();

    loop {
        print_board(&board);

        let status = check_board_status(&board);
        if status != GameStatus::None {
            match status {
                GameStatus::Draw => println!("Draw!"),
                GameStatus::X => println!("Player X has won!"),
                _ => println!("Player O has won!"),
            }
            return;
        }

        let mut x: usize = to_number(&get_input()).try_into().unwrap();
        while x > 8 {
            x = to_number(&get_input()).try_into().unwrap();
        }

        // dont know how to do it properly, but it looks like it does
        match whose_turn {
            &Player::X => {
                insert_symbol_to_board(x, Player::X, &mut board);
                whose_turn = &Player::O;
            }
            _ => {
                insert_symbol_to_board(x, Player::O, &mut board);
                whose_turn = &Player::X;
            }
        }
    }
}

fn main() {
    let mut play_game = true;

    while play_game {
        game();

        let play_next: bool = get_input().trim().parse().ok().unwrap();
        if !play_next {
            play_game = false;
        }
    }
}
