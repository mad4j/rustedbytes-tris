
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, PathBuilder, Source};

mod consts;

#[derive(Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Occupied(Player),
}

struct Game {
    board: [[Cell; 3]; 3],
    current_player: Player,
    moves: Vec<(usize, usize)>,
    game_over: bool,
}

impl Game {
    fn new() -> Self {
        Self {
            board: [[Cell::Empty; 3]; 3],
            current_player: Player::X,
            moves: Vec::new(),
            game_over: false,
        }
    }

    fn reset(&mut self) {
        self.board = [[Cell::Empty; 3]; 3];
        self.current_player = Player::X;
        self.moves.clear();
        self.game_over = false;
    }

    fn make_move(&mut self, x: usize, y: usize) {
        if self.game_over || self.board[y][x] != Cell::Empty {
            return;
        }

        self.board[y][x] = Cell::Occupied(self.current_player);
        self.moves.push((x, y));

        if self.moves.len() > 6 {
            let (old_x, old_y) = self.moves.remove(0);
            self.board[old_y][old_x] = Cell::Empty;
        }

        if self.check_winner() {
            self.game_over = true;
        } else {
            self.current_player = match self.current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
        }
    }

    fn check_winner(&self) -> bool {
        let lines = [
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

        for line in &lines {
            if let [Cell::Occupied(p1), Cell::Occupied(p2), Cell::Occupied(p3)] = [
                self.board[line[0].1][line[0].0],
                self.board[line[1].1][line[1].0],
                self.board[line[2].1][line[2].0],
            ] {
                if p1 == p2 && p2 == p3 {
                    return true;
                }
            }
        }

        false
    }
}


fn draw_grid(draw_target: &mut DrawTarget) {
    // Draw vertical lines

    for i in 1..3 {
        let offset = (i * consts::CELL_SIZE) as f32;
    
        // Vertical line
        let mut pb = PathBuilder::new();
        pb.move_to(offset, 0.0);
        pb.line_to(offset, consts::HEIGHT as f32);
        let path = pb.finish();
        draw_target.stroke(
            &path,
            &Source::Solid(consts::COLOR_BLUE),
            &consts::LINE_STROKE_STYLE,
            &DrawOptions::default(),
        );

        // Horizontal line
        let mut pb = PathBuilder::new();
        pb.move_to(0.0, offset);
        pb.line_to(consts::WIDTH as f32, offset);
        let path = pb.finish();
        draw_target.stroke(
            &path,
            &Source::Solid(consts::COLOR_BLUE),
            &consts::LINE_STROKE_STYLE,
            &DrawOptions::default(),
        );
    }
}


fn main() {

    let mut window = Window::new(
        "Wanishing Tris",
        consts::WIDTH,
        consts::HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut draw_target = DrawTarget::new(consts::WIDTH as i32, consts::HEIGHT as i32);
    let mut game = Game::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.get_mouse_down(MouseButton::Left) {
            if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Clamp) {
                let x = (mouse_x as usize) / consts::CELL_SIZE;
                let y = (mouse_y as usize) / consts::CELL_SIZE;

                if x < 3 && y < 3 {
                    game.make_move(x, y);
                }
            }
        }

        if window.is_key_down(Key::R) {
            game.reset();
        }

        // Clear the draw target
        draw_target.clear(consts::COLOR_WHITE);

        // Draw grid
        draw_grid(&mut draw_target);
        
        
        // Draw pieces
        for y in 0..3 {
            for x in 0..3 {
                let cell = game.board[y][x];
                let start_x = (x * consts::CELL_SIZE) as f32;
                let start_y = (y * consts::CELL_SIZE) as f32;

                match cell {
                    Cell::Occupied(Player::X) => {
                        let offset = 0.2 * consts::CELL_SIZE as f32;
                        // Draw X
                        let mut pb = PathBuilder::new();
                        pb.move_to(start_x + offset, start_y + offset);
                        pb.line_to(
                            start_x + consts::CELL_SIZE as f32 - offset,
                            start_y + consts::CELL_SIZE as f32 - offset,
                        );
                        let path = pb.finish();
                        draw_target.stroke(
                            &path,
                            &Source::Solid(consts::COLOR_RED),
                            &consts::SYMBOL_STROKE_STYLE,
                            &DrawOptions::default(),
                        );

                        let mut pb = PathBuilder::new();
                        pb.move_to(start_x + consts::CELL_SIZE as f32 - offset, start_y + offset);
                        pb.line_to(start_x + offset, start_y + consts::CELL_SIZE as f32 - offset);
                        let path = pb.finish();
                        draw_target.stroke(
                            &path,
                            &Source::Solid(consts::COLOR_RED),
                            &consts::SYMBOL_STROKE_STYLE,
                            &DrawOptions::default(),
                        );
                    }
                    Cell::Occupied(Player::O) => {
                        // Draw O
                        let mut pb = PathBuilder::new();
                        pb.arc(
                            start_x + 0.5 * consts::CELL_SIZE as f32,
                            start_y + 0.5 * consts::CELL_SIZE as f32,
                            0.3 * consts::CELL_SIZE as f32,
                            0.0,
                            2.0 * std::f32::consts::PI,
                        );
                        let path = pb.finish();
                        draw_target.stroke(
                            &path,
                            &Source::Solid(consts::COLOR_ORANGE),
                            &consts::SYMBOL_STROKE_STYLE,
                            &DrawOptions::default(),
                        );
                    }
                    _ => {}
                }
            }
        }

        if game.game_over {
            let winning_lines = [
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

            for line in &winning_lines {
                if let [Cell::Occupied(p1), Cell::Occupied(p2), Cell::Occupied(p3)] = [
                    game.board[line[0].1][line[0].0],
                    game.board[line[1].1][line[1].0],
                    game.board[line[2].1][line[2].0],
                ] {
                    if p1 == p2 && p2 == p3 {
                        let start_x =
                            (line[0].0 * consts::CELL_SIZE + consts::CELL_SIZE / 2) as f32;
                        let start_y =
                            (line[0].1 * consts::CELL_SIZE + consts::CELL_SIZE / 2) as f32;
                        let end_x = (line[2].0 * consts::CELL_SIZE + consts::CELL_SIZE / 2) as f32;
                        let end_y = (line[2].1 * consts::CELL_SIZE + consts::CELL_SIZE / 2) as f32;

                        let mut pb = PathBuilder::new();
                        pb.move_to(start_x, start_y);
                        pb.line_to(end_x, end_y);
                        let path = pb.finish();
                        draw_target.stroke(
                            &path,
                            &Source::Solid(consts::COLOR_GREEN),
                            &consts::LINE_STROKE_STYLE,
                            &consts::DRAW_OPTIONS,
                        );
                        break;
                    }
                }
            }
        }

        window
            .update_with_buffer(draw_target.get_data(), consts::WIDTH, consts::HEIGHT)
            .unwrap();
    }
}
