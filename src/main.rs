/*!
 * Vanishing Tris implementation
 * A simple Tic-Tac-Toe game with a twist: the symbols vanishes after the third move.
 * The game is played on a 3x3 grid, and the first player to align three symbols wins.
 * The game also includes an AI opponent that plays as Player O.
 * The game is implemented using the `minifb` library for window management and `raqote` for drawing.
 * The game can be reset by pressing the space bar or right-clicking the mouse.
 */

mod ai;
mod consts;
mod draws;
mod game;

use minifb::{Icon, Key, MouseButton, MouseMode, Window, WindowOptions};
use raqote::DrawTarget;
use std::str::FromStr;

use consts::{CELL_SIZE, HEIGHT, WIDTH};
use draws::{clear_background, draw_grid, draw_player_o, draw_player_x, draw_winning_line};
use game::{Cell, Game, Player};


// The icon file is included as a byte array for Windows
#[cfg(target_os = "windows")]
static ICO_FILE: &[u8] = include_bytes!("../assets/app.ico");


// The main function initializes the game window, sets up the game logic, and handles user input.
// It also manages the game loop, updating the display and checking for game state changes.
// The game loop continues until the window is closed or the escape key is pressed.
// The game can be reset by pressing the space bar or right-clicking the mouse.
// The AI opponent makes a move if it's Player O's turn, and the player can make a move by clicking on the grid.
fn main() {
    
    // Create a new window with the specified title, width, height, and options
    let mut window = Window::new(
        &format!("Vanishing Tic-Tac-Toe - v{}", env!("CARGO_PKG_VERSION")),
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Set the window icon
    #[cfg(target_os = "windows")]
    {
        let temp_file = temp_file::with_contents(ICO_FILE);
        window.set_icon(Icon::from_str(temp_file.path().to_str().unwrap()).unwrap());
    }

    // Limit the frame rate to 60 FPS
    window.set_target_fps(60);

    // Set up canvas and game logic
    let mut draw_target = DrawTarget::new(WIDTH as i32, HEIGHT as i32);
    let mut game = Game::new();

    // Game main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {

        // check for player moves only if the game is not over
        if !game.is_over() {
            // If it's Player O's turn, let the AI make a move
            if game.get_current_player() == Player::O {
                std::thread::sleep(std::time::Duration::from_millis(100));
                if let Some((x, y)) = ai::get_next_move(&game) {
                    game.make_move(x, y);
                }
            // Otherwise, check for mouse input
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

        // Check for game reset request
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
                let cell = game.get_board_cell(x, y);

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
            draw_winning_line(&mut draw_target, &game.get_winning_line().unwrap());
        }

        // Update the window with the draw target
        window
            .update_with_buffer(draw_target.get_data(), WIDTH, HEIGHT)
            .unwrap();
    }
}
