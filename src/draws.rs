/*! This module provides drawing functions for a tic-tac-toe game using the `raqote` graphics library.
 *  It includes functions to draw the game grid, player symbols (X and O), a winning line, and to clear the background.
 *  The drawing is performed on a `DrawTarget` object, which represents the canvas for rendering.
 */

use raqote::{DrawTarget, PathBuilder, Source};

use crate::consts::{
    CELL_PADDING, CELL_SIZE, COLOR_BLUE, COLOR_GRAY, COLOR_ORANGE, COLOR_RED, COLOR_WHITE, DRAW_OPTIONS, GRID_LINE_STROKE_STYLE, HEIGHT, SYMBOL_STROKE_STYLE, WIDTH, WIN_LINE_STROKE_STYLE
};


/// Draws the game grid on the provided `DrawTarget`.
/// The grid consists of vertical and horizontal lines that divide the canvas into a 3x3 grid.
pub fn draw_grid(draw_target: &mut DrawTarget) {
    // Draw vertical lines

    for i in 1..3 {
        let offset = (CELL_PADDING + i * CELL_SIZE) as f32;

        // Vertical line
        let mut pb = PathBuilder::new();
        pb.move_to(offset, 0.0 + CELL_PADDING as f32);
        pb.line_to(offset, HEIGHT as f32 - CELL_PADDING as f32);
        let path = pb.finish();
        draw_target.stroke(
            &path,
            &Source::Solid(COLOR_GRAY),
            &GRID_LINE_STROKE_STYLE,
            &DRAW_OPTIONS,
        );

        // Horizontal line
        let mut pb = PathBuilder::new();
        pb.move_to(0.0 + CELL_PADDING as f32, offset);
        pb.line_to(WIDTH as f32 - CELL_PADDING as f32, offset);
        let path = pb.finish();
        draw_target.stroke(
            &path,
            &Source::Solid(COLOR_GRAY),
            &GRID_LINE_STROKE_STYLE,
            &DRAW_OPTIONS,
        );
    }
}


/// Draws the symbol for Plaryer X at the specified coordinates (x, y) on the provided `DrawTarget`.
/// The symbol is drawn as two diagonal lines forming an "X".
pub fn draw_player_x(draw_target: &mut DrawTarget, x: usize, y: usize) {
    let start_x = (CELL_PADDING + x * CELL_SIZE) as f32;
    let start_y = (CELL_PADDING + y * CELL_SIZE) as f32;
    let offset = 0.2 * CELL_SIZE as f32;

    // Draw X
    let mut pb = PathBuilder::new();
    pb.move_to(start_x + offset, start_y + offset);
    pb.line_to(
        start_x + CELL_SIZE as f32 - offset,
        start_y + CELL_SIZE as f32 - offset,
    );
    let path = pb.finish();
    draw_target.stroke(
        &path,
        &Source::Solid(COLOR_RED),
        &SYMBOL_STROKE_STYLE,
        &DRAW_OPTIONS,
    );

    let mut pb = PathBuilder::new();
    pb.move_to(start_x + CELL_SIZE as f32 - offset, start_y + offset);
    pb.line_to(start_x + offset, start_y + CELL_SIZE as f32 - offset);
    let path = pb.finish();
    draw_target.stroke(
        &path,
        &Source::Solid(COLOR_RED),
        &SYMBOL_STROKE_STYLE,
        &DRAW_OPTIONS,
    );
}


/// Draws the symbol for Player O at the specified coordinates (x, y) on the provided `DrawTarget`.
/// The symbol is drawn as a circle.
pub fn draw_player_o(draw_target: &mut DrawTarget, x: usize, y: usize) {
    let start_x = (CELL_PADDING + x * CELL_SIZE) as f32;
    let start_y = (CELL_PADDING + y * CELL_SIZE) as f32;

    // Draw O
    let mut pb = PathBuilder::new();
    pb.arc(
        start_x + 0.5 * CELL_SIZE as f32,
        start_y + 0.5 * CELL_SIZE as f32,
        0.3 * CELL_SIZE as f32,
        0.0,
        2.0 * std::f32::consts::PI,
    );
    let path = pb.finish();
    draw_target.stroke(
        &path,
        &Source::Solid(COLOR_ORANGE),
        &SYMBOL_STROKE_STYLE,
        &DRAW_OPTIONS,
    );
}


/// Draws a winning line on the provided `DrawTarget`.
/// The line connects the start and end coordinates of the winning line.
/// The coordinates are specified in a 2D array format, where each element represents a cell in the grid.
pub fn draw_winning_line(draw_target: &mut DrawTarget, line: &[(usize, usize); 3]) {
    let start_x = (line[0].0 * CELL_SIZE + CELL_SIZE / 2 + CELL_PADDING) as f32;
    let start_y = (line[0].1 * CELL_SIZE + CELL_SIZE / 2 + CELL_PADDING) as f32;
    let end_x = (line[2].0 * CELL_SIZE + CELL_SIZE / 2 + CELL_PADDING) as f32;
    let end_y = (line[2].1 * CELL_SIZE + CELL_SIZE / 2 + CELL_PADDING) as f32;

    let mut pb = PathBuilder::new();
    pb.move_to(start_x, start_y);
    pb.line_to(end_x, end_y);
    let path = pb.finish();
    draw_target.stroke(
        &path,
        &Source::Solid(COLOR_BLUE),
        &WIN_LINE_STROKE_STYLE,
        &DRAW_OPTIONS,
    );
}


/// Clears the background of the provided `DrawTarget`.
/// The background is filled with a solid white color.
/// This function is typically called at the beginning of each frame to reset the canvas.
/// It ensures that the previous drawings are cleared before rendering the new frame.
pub fn clear_background(draw_target: &mut DrawTarget) {
    draw_target.clear(COLOR_WHITE);
}
