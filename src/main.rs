mod consts;
mod draws;

use minifb::{Icon, Key, MouseButton, MouseMode, Window, WindowOptions};
use raqote::DrawTarget;
use std::str::FromStr;

use consts::{CELL_SIZE, HEIGHT, WIDTH};
use draws::{clear_background, draw_grid, draw_player_o, draw_player_x, draw_winning_line};
use rand::seq::IteratorRandom;

#[cfg(target_os = "windows")]
static ICO_FILE: &[u8] = include_bytes!("../assets/app.ico");

static LINES: [[(usize, usize); 3]; 8] = [
    // Rows
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    // Columns
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    // Diagonals
    [(0, 0), (1, 1), (2, 2)],
    [(2, 0), (1, 1), (0, 2)],
];

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Occupied(Player),
}

#[derive(Debug, Clone)]
struct Game {
    board: [[Cell; 3]; 3],
    current_player: Player,
    moves: Vec<(usize, usize)>,
    winning_line: Option<[(usize, usize); 3]>,
}

impl Game {
    fn new() -> Self {
        Self {
            board: [[Cell::Empty; 3]; 3],
            current_player: Player::X,
            moves: Vec::new(),
            winning_line: None,
        }
    }

    fn reset(&mut self) {

        // Reset the game state
        self.board = [[Cell::Empty; 3]; 3];
        self.moves.clear();
        self.winning_line = None;
        self.current_player = Player::X;
    }

    fn make_move(&mut self, x: usize, y: usize) {

        // Check if the game is over or the cell is already occupied
        if self.is_over() || self.board[y][x] != Cell::Empty {
            return;
        }

        // Place the current player's symbol on the board
        self.board[y][x] = Cell::Occupied(self.current_player);
        self.moves.push((x, y));

        // Remove the oldest move if there are more than 6 moves
        if self.moves.len() > 6 {
            let (old_x, old_y) = self.moves.remove(0);
            self.board[old_y][old_x] = Cell::Empty;
        }

        // Check for a winner
        self.check_winner();

        // Switch players if the game is not over
        if !self.is_over() {
            self.switch_player();
        }
    }

    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }

    fn check_winner(&mut self) {
        for line in &LINES {
            if let [Cell::Occupied(p1), Cell::Occupied(p2), Cell::Occupied(p3)] = [
                self.board[line[0].1][line[0].0],
                self.board[line[1].1][line[1].0],
                self.board[line[2].1][line[2].0],
            ] {
                if p1 == p2 && p2 == p3 {
                    self.winning_line = Some(*line);
                    return;
                }
            }
        }

        self.winning_line = None;
    }

    fn is_over(&self) -> bool {
        self.winning_line.is_some()
    }
}

fn get_next_move(game: &Game) -> Option<(usize, usize)> {
    let mut rng = rand::rng();

    // Check if any move leads to a win
    for y in 0..3 {
        for x in 0..3 {
            if game.board[y][x] == Cell::Empty {
                let mut simulated_game = game.clone();
                simulated_game.make_move(x, y);
                if simulated_game.is_over() {
                    return Some((x, y));
                }
            }
        }
    }

    // Check if any move prevents the opponent from winning
    for y in 0..3 {
        for x in 0..3 {
            if game.board[y][x] == Cell::Empty {
                let mut simulated_game = game.clone();
                simulated_game.switch_player();
                simulated_game.make_move(x, y);
                if simulated_game.is_over() {
                    return Some((x, y));
                }
            }
        }
    }

    // Otherwise, pick a random move
    (0..3)
        .flat_map(|y| (0..3).map(move |x| (x, y)))
        .filter(|&(x, y)| game.board[y][x] == Cell::Empty)
        .choose(&mut rng)
}

fn main() {
    let mut window = Window::new("Vanishing Tris", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let mut draw_target = DrawTarget::new(WIDTH as i32, HEIGHT as i32);
    let mut game = Game::new();

    #[cfg(target_os = "windows")]
    {
        let temp_file = temp_file::with_contents(ICO_FILE);
        window.set_icon(Icon::from_str(temp_file.path().to_str().unwrap()).unwrap());
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if !game.is_over() {
            if game.current_player == Player::O {
                std::thread::sleep(std::time::Duration::from_millis(100));
                if let Some((x, y)) = get_next_move(&game) {
                    game.make_move(x, y);
                }
            } else if window.get_mouse_down(MouseButton::Left) {
                if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Clamp) {
                    let x = (mouse_x as usize) / CELL_SIZE;
                    let y = (mouse_y as usize) / CELL_SIZE;

                    if x < 3 && y < 3 {
                        game.make_move(x, y);
                    }
                }
            }
        }

        if window.is_key_down(Key::Space) || window.get_mouse_down(MouseButton::Right) {
            game.reset();
        }

        // Clear the draw target
        clear_background(&mut draw_target);

        // Draw grid
        draw_grid(&mut draw_target);

        // Draw pieces
        for y in 0..3 {
            for x in 0..3 {
                let cell = game.board[y][x];

                match cell {
                    Cell::Occupied(Player::X) => {
                        draw_player_x(&mut draw_target, x, y);
                    }
                    Cell::Occupied(Player::O) => {
                        draw_player_o(&mut draw_target, x, y);
                    }
                    _ => {}
                }
            }
        }

        // Draw winning line if the game is over
        if game.is_over() {
            draw_winning_line(&mut draw_target, &game.winning_line.unwrap());
        }

        // Update the window with the draw target
        window
            .update_with_buffer(draw_target.get_data(), WIDTH, HEIGHT)
            .unwrap();
    }
}
