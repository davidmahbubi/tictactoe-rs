use std::{io::{stdin, stdout, Stdin, Write}, process::exit};

#[derive(Debug, Clone, PartialEq)]
enum Player {
    X,
    O,
    NONE
}

type BoardState = [[Player; 3]; 3];

fn main() {
    println!("Welcome to Rust Tic Tac Toe");

    let mut board_condition: BoardState = empty_board();
    let mut player_turn: Player = Player::X;

    let stdin: Stdin = stdin();
    let input: &mut String = &mut String::new();

    render_board(&board_condition);

    'gameloop: loop {
        input.clear();

        println!("Current player : {:?}", player_turn);
        print!("Please input your position X,Y (e.g 1,3) : ");

        let _ = stdout().flush();
        let _ = stdin.read_line(input);

        if input == "\n" {
            println!("Please provide a valid input!");
        } else {
            if input.ends_with("\n") {
                input.pop();
            }
            
            let expected_post: Vec<&str> = input.split(',').collect();

            let x: usize = (expected_post[0].trim()).parse::<usize>().unwrap() - 1;
            let y: usize = (expected_post[1].trim()).parse::<usize>().unwrap() - 1;

            if x > 2 || y > 2 {
                println!("Invalid value! x or y cannot have more than 3 value!");
                continue 'gameloop;
            }

            if !matches!(board_condition[y][x], Player::NONE) {
                println!("Invalid input: {},{} already filled by {:?}", x + 1, y + 1, board_condition[y][x]);
                continue 'gameloop;
            }

            board_condition[y][x] = player_turn.clone();

            if matches!(player_turn, Player::X) {
                player_turn = Player::O;
            } else {
                player_turn = Player::X;
            }
        }

        render_board(&board_condition);
        
        let winner: Option<Player> = check_winner(&board_condition);

        match winner {
            Some(player) => {
                println!("CONGRATULATION! \nPlayer {:?} is win", player);
                exit(0);
            },
            None => (),
        }

        if check_draw(&board_condition) {
            println!("No one won!");
            exit(0);
        }
    };

}

fn render_board(board_state: &BoardState) {
    println!("*---*---*---*");
    for board_row in board_state.iter() {
        for (i, item) in board_row.iter().enumerate() {

            if i == 0 {
                print!("| ")
            } else if i == 1 {
                print!(" | ")
            } else if i == 2 {
                print!(" | ")
            }

            let item_str: String = match item {
                Player::NONE => String::from("-"),
                Player::X => String::from("X"),
                Player::O => String::from("O"),
            };

            print!("{item_str}");

            if i == 2 {
                print!(" |")
            }
        }
        
        println!("\n*---*---*---*");
    }
}

fn empty_board() -> BoardState {
    [
        [Player::NONE, Player::NONE, Player::NONE],
        [Player::NONE, Player::NONE, Player::NONE],
        [Player::NONE, Player::NONE, Player::NONE],
    ]
}

fn check_winner(board_state: &BoardState) -> Option<Player> {
    // Check vertical state
    for y in 0..3 {
        if board_state[y][0] != Player::NONE
            && board_state[y][0] == board_state[y][1]
            && board_state[y][1] == board_state[y][2]
        {
            return Some(board_state[y][0].clone());
        }
    }

    // Check horizontal state
    for x in 0..3 {
        if board_state[0][x] != Player::NONE
            && board_state[0][x] == board_state[1][x]
            && board_state[1][x] == board_state[2][x]
        {
            return Some(board_state[0][x].clone());
        }
    }

    // Check diagonal state
    if board_state[1][1] != Player::NONE {
        // Top-left to bottom-right
        if board_state[0][0] == board_state[1][1]
            && board_state[1][1] == board_state[2][2]
        {
            return Some(board_state[1][1].clone());
        }
        // Top-right to bottom-left
        if board_state[0][2] == board_state[1][1]
            && board_state[1][1] == board_state[2][0]
        {
            return Some(board_state[1][1].clone());
        }
    }

    None
}

fn check_draw(board_state: &BoardState) -> bool {
    for board_row in board_state.iter() {
        for board_item in board_row.iter() {
            if *board_item == Player::NONE {
                return false;
            }
        }
    }
    true
}