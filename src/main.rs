use std::{error::Error, io::stdout, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Terminal,
};

#[derive(Debug, Clone, PartialEq)]
enum Player {
    X,
    O,
    NONE
}

type BoardState = [[Player; 3]; 3];

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to Rust Tic Tac Toe - press 'q' to quit");

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_game(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{err}");
    }

    Ok(())
}

fn run_game(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<(), Box<dyn Error>> {
    let mut board_condition: BoardState = empty_board();
    let mut player_turn: Player = Player::X;

    loop {
        terminal.draw(|f| render_board(f, &board_condition))?;

        if event::poll(Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(k) if k.code == KeyCode::Char('q') => return Ok(()),
                Event::Mouse(m) if matches!(m.kind, MouseEventKind::Up(_)) => {
                    if let Some((x, y)) = mouse_to_cell(f.size(), m.column, m.row) {
                        if matches!(board_condition[y][x], Player::NONE) {
                            board_condition[y][x] = player_turn.clone();
                            player_turn = if player_turn == Player::X { Player::O } else { Player::X };

                            if let Some(p) = check_winner(&board_condition) {
                                terminal.draw(|f| render_board(f, &board_condition))?;
                                println!("Player {:?} wins!", p);
                                return Ok(());
                            }
                            if check_draw(&board_condition) {
                                terminal.draw(|f| render_board(f, &board_condition))?;
                                println!("It's a draw!");
                                return Ok(());
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn render_board<B: tui::backend::Backend>(f: &mut tui::Frame<B>, board_state: &BoardState) {
    let area = f.size();
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(33); 3])
        .split(area);
    for (y, row) in rows.iter().enumerate() {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33); 3])
            .split(*row);
        for (x, col) in cols.iter().enumerate() {
            let cell = match board_state[y][x] {
                Player::X => "X",
                Player::O => "O",
                Player::NONE => "",
            };
            let block = Block::default().borders(Borders::ALL).title(cell);
            f.render_widget(block, *col);
        }
    }
}

fn mouse_to_cell(area: Rect, column: u16, row: u16) -> Option<(usize, usize)> {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(33); 3])
        .split(area);
    for (y, r) in rows.iter().enumerate() {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(33); 3])
            .split(*r);
        for (x, c) in cols.iter().enumerate() {
            if column >= c.x && column < c.x + c.width && row >= c.y && row < c.y + c.height {
                return Some((x, y));
            }
        }
    }
    None
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