use std::{error::Error, io::stdout, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
    NONE
}

type BoardState = [[Player; 3]; 3];

#[derive(Debug, Clone, Copy)]
enum Difficulty {
    Easy,
    Hard,
}

#[derive(Debug, Clone, Copy)]
enum GameMode {
    HumanVsHuman,
    VsComputer(Difficulty),
}

fn main() -> Result<(), Box<dyn Error>> {
    let mode = match std::env::args().nth(1).as_deref() {
        Some("easy") => GameMode::VsComputer(Difficulty::Easy),
        Some("hard") => GameMode::VsComputer(Difficulty::Hard),
        _ => GameMode::HumanVsHuman,
    };

    println!("Welcome to Rust Tic Tac Toe - press 'q' to quit");
    if let GameMode::VsComputer(diff) = mode {
        println!("Playing vs computer - {:?} difficulty", diff);
    }

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_game(&mut terminal, mode);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{err}");
    }

    Ok(())
}

fn run_game(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    mode: GameMode,
) -> Result<(), Box<dyn Error>> {
    let mut board_condition: BoardState = empty_board();
    let mut player_turn: Player = Player::X;
    let mut rng_seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    let (computer, difficulty) = match mode {
        GameMode::VsComputer(diff) => (Some(Player::O), Some(diff)),
        GameMode::HumanVsHuman => (None, None),
    };

    loop {
        terminal.draw(|f| render_board(f, &board_condition))?;

        if let (Some(comp), Some(diff)) = (computer, difficulty) {
            if player_turn == comp {
                if let Some((cx, cy)) = ai_move(&board_condition, diff, comp, &mut rng_seed) {
                    board_condition[cy][cx] = comp;
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
                    player_turn = Player::X;
                    continue;
                }
            }
        }

        if event::poll(Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(k) if k.code == KeyCode::Char('q') => return Ok(()),
                Event::Mouse(m) if matches!(m.kind, MouseEventKind::Up(_)) => {
                    if let Some((x, y)) = mouse_to_cell(terminal.size()?, m.column, m.row) {
                        if matches!(board_condition[y][x], Player::NONE) {
                            board_condition[y][x] = player_turn;

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

                            player_turn = if player_turn == Player::X { Player::O } else { Player::X };
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn render_board<B: tui::backend::Backend>(f: &mut tui::Frame<B>, board_state: &BoardState) {
    let area = centered_rect(30, 15, f.size());
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5); 3])
        .split(area);
    for (y, row) in rows.iter().enumerate() {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(10); 3])
            .split(*row);
        for (x, col) in cols.iter().enumerate() {
            let block = Block::default().borders(Borders::ALL);
            let inner = block.inner(*col);
            f.render_widget(block, *col);

            let cell = match board_state[y][x] {
                Player::X => "X",
                Player::O => "O",
                Player::NONE => "",
            };
            if !cell.is_empty() {
                let area = Rect::new(inner.x, inner.y + inner.height / 2, inner.width, 1);
                let paragraph = Paragraph::new(cell).alignment(Alignment::Center);
                f.render_widget(paragraph, area);
            }
        }
    }
}

fn mouse_to_cell(area: Rect, column: u16, row: u16) -> Option<(usize, usize)> {
    let board = centered_rect(30, 15, area);
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5); 3])
        .split(board);
    for (y, r) in rows.iter().enumerate() {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(10); 3])
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

fn lcg_rand(seed: &mut u64) -> usize {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    (*seed >> 32) as usize
}

fn ai_move(
    board: &BoardState,
    diff: Difficulty,
    ai_player: Player,
    seed: &mut u64,
) -> Option<(usize, usize)> {
    match diff {
        Difficulty::Easy => {
            let mut empty = Vec::new();
            for y in 0..3 {
                for x in 0..3 {
                    if board[y][x] == Player::NONE {
                        empty.push((x, y));
                    }
                }
            }
            if empty.is_empty() {
                None
            } else {
                let idx = lcg_rand(seed) % empty.len();
                Some(empty[idx])
            }
        }
        Difficulty::Hard => best_move(board, ai_player),
    }
}

fn best_move(board: &BoardState, ai: Player) -> Option<(usize, usize)> {
    let mut best_score = -2;
    let mut best = None;
    let mut board_mut = *board;
    for y in 0..3 {
        for x in 0..3 {
            if board_mut[y][x] == Player::NONE {
                board_mut[y][x] = ai;
                let score = minimax(&mut board_mut, opposite(ai), false, ai);
                board_mut[y][x] = Player::NONE;
                if score > best_score {
                    best_score = score;
                    best = Some((x, y));
                }
            }
        }
    }
    best
}

fn minimax(board: &mut BoardState, player: Player, maximizing: bool, ai: Player) -> i32 {
    if let Some(w) = check_winner(board) {
        return if w == ai { 1 } else { -1 };
    }
    if check_draw(board) {
        return 0;
    }
    let mut best = if maximizing { -2 } else { 2 };
    for y in 0..3 {
        for x in 0..3 {
            if board[y][x] == Player::NONE {
                board[y][x] = player;
                let score = minimax(board, opposite(player), !maximizing, ai);
                board[y][x] = Player::NONE;
                if maximizing {
                    if score > best {
                        best = score;
                    }
                } else if score < best {
                    best = score;
                }
            }
        }
    }
    best
}

fn opposite(p: Player) -> Player {
    if p == Player::X { Player::O } else { Player::X }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let w = width.min(area.width);
    let h = height.min(area.height);
    let x = area.x + (area.width - w) / 2;
    let y = area.y + (area.height - h) / 2;
    Rect::new(x, y, w, h)
}
