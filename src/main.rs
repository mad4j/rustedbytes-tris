use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

const WIDTH: usize = 300;
const HEIGHT: usize = 300;
const CELL_SIZE: usize = 100;

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
            if let [Cell::Occupied(p1), Cell::Occupied(p2), Cell::Occupied(p3)] =
                [self.board[line[0].1][line[0].0], self.board[line[1].1][line[1].0], self.board[line[2].1][line[2].0]]
            {
                if p1 == p2 && p2 == p3 {
                    return true;
                }
            }
        }

        false
    }
}

fn main() {
    let mut window = Window::new(
        "Tris - Rust",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut draw_target = DrawTarget::new(WIDTH as i32, HEIGHT as i32);
    let mut game = Game::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.get_mouse_down(MouseButton::Left) {
            if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Clamp) {
                let x = (mouse_x as usize) / CELL_SIZE;
                let y = (mouse_y as usize) / CELL_SIZE;

                if x < 3 && y < 3 {
                    game.make_move(x, y);
                }
            }
        }

        if window.is_key_down(Key::R) {
            game.reset();
        }

        // Clear the draw target
        draw_target.clear(SolidSource {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        });

        // Draw grid
        let black = Source::Solid(SolidSource {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        });

        for i in 1..3 {
            let offset = (i * CELL_SIZE) as f32;

            // Vertical line
            let mut pb = PathBuilder::new();
            pb.move_to(offset, 0.0);
            pb.line_to(offset, HEIGHT as f32);
            let path = pb.finish();
            draw_target.stroke(&path, &black, &raqote::StrokeStyle::default(), &DrawOptions::default());

            // Horizontal line
            let mut pb = PathBuilder::new();
            pb.move_to(0.0, offset);
            pb.line_to(WIDTH as f32, offset);
            let path = pb.finish();
            draw_target.stroke(&path, &black, &raqote::StrokeStyle::default(), &DrawOptions::default());
        }

        // Draw pieces
        for y in 0..3 {
            for x in 0..3 {
                let cell = game.board[y][x];
                let start_x = (x * CELL_SIZE) as f32;
                let start_y = (y * CELL_SIZE) as f32;

                match cell {
                    Cell::Occupied(Player::X) => {
                        let red = Source::Solid(SolidSource {
                            r: 255,
                            g: 0,
                            b: 0,
                            a: 255,
                        });

                        // Draw X
                        let mut pb = PathBuilder::new();
                        pb.move_to(start_x, start_y);
                        pb.line_to(start_x + CELL_SIZE as f32, start_y + CELL_SIZE as f32);
                        let path = pb.finish();
                        draw_target.stroke(&path, &red, &raqote::StrokeStyle::default(), &DrawOptions::default());

                        let mut pb = PathBuilder::new();
                        pb.move_to(start_x + CELL_SIZE as f32, start_y);
                        pb.line_to(start_x, start_y + CELL_SIZE as f32);
                        let path = pb.finish();
                        draw_target.stroke(&path, &red, &raqote::StrokeStyle::default(), &DrawOptions::default());
                    }
                    Cell::Occupied(Player::O) => {
                        let blue = Source::Solid(SolidSource {
                            r: 0,
                            g: 0,
                            b: 255,
                            a: 255,
                        });

                        // Draw O
                        let mut pb = PathBuilder::new();
                        pb.arc(
                            start_x + CELL_SIZE as f32 / 2.0,
                            start_y + CELL_SIZE as f32 / 2.0,
                            CELL_SIZE as f32 / 2.0 - 5.0,
                            0.0,
                            2.0 * std::f32::consts::PI,
                        );
                        let path = pb.finish();
                        draw_target.stroke(&path, &blue, &raqote::StrokeStyle::default(), &DrawOptions::default());
                    }
                    _ => {}
                }
            }
        }

        window
            .update_with_buffer(draw_target.get_data(), WIDTH, HEIGHT)
            .unwrap();
    }
}