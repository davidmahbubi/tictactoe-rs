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
            // [0] = x
            // [1] = y
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
    // Diagonal Checking
    let diagonals: [[[usize; 2]; 3]; 2] = [
        [[0,0], [1,1], [2,2]],
        [[2,0], [1,1], [0,2]]
    ];

    let mut winner: Option<Player> = None;

    'diagonal_loop: for diagonal in diagonals.iter() {
        for (i, diagonal_point) in diagonal.iter().enumerate() {

            let previous_state: Option<Player> = if i == 0 { None } else { Some(board_state[diagonal[i - 1][0]][diagonal[i - 1][1]].clone()) };
            let diagonal_state: Player = board_state[diagonal_point[0]][diagonal_point[1]].clone();
            
            if diagonal_state == Player::NONE {
                continue 'diagonal_loop;
            }

            if previous_state != None {
                let is_same_as_previous = match previous_state {
                    Some(player) => if player == diagonal_state { true } else { false },
                    None => false,
                };
    
                if !is_same_as_previous {
                    continue 'diagonal_loop;
                }
    
                if i == 2 {
                    winner = Some(diagonal_state);
                    return winner;
                }
            }
        }
    };

    'x_loop: for x in 0..2 {
        for y in 0..2 {
            let previous_state: Option<Player> = if y == 0 { None } else { Some(board_state[x][y - 1].clone()) };
            let vertical_state: Player = board_state[x][y].clone();

            if previous_state != None {
                let is_same_as_previous = match previous_state {
                    Some(player) => if player == vertical_state { true } else { false },
                    None => false,
                };

                if !is_same_as_previous {
                    continue 'x_loop;
                }

                if y == 2 {
                    winner = Some(vertical_state);
                    return winner;
                }
            }
        }

        'y_loop: for y in 0..2 {
            for x in 0..2 {
                let previous_state: Option<Player> = if x == 0 { None } else { Some(board_state[y][x - 1].clone()) };
                let vertical_state: Player = board_state[y][x].clone();
    
                if previous_state != None {
                    let is_same_as_previous = match previous_state {
                        Some(player) => if player == vertical_state { true } else { false },
                        None => false,
                    };
    
                    if !is_same_as_previous {
                        continue 'y_loop;
                    }
    
                    if x == 2 {
                        winner = Some(vertical_state);
                        return winner;
                    }
                }
            }
        }
    }
    
    return winner;
}