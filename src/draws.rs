use raqote::{DrawTarget, PathBuilder, Source};

use crate::consts::{
    CELL_PADDING, CELL_SIZE, COLOR_BLUE, COLOR_GRAY, COLOR_ORANGE, COLOR_RED, COLOR_WHITE,
    DRAW_OPTIONS, HEIGHT, LINE_STROKE_STYLE, SYMBOL_STROKE_STYLE, WIDTH,
};

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
            &LINE_STROKE_STYLE,
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
            &LINE_STROKE_STYLE,
            &DRAW_OPTIONS,
        );
    }
}

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
        &LINE_STROKE_STYLE,
        &DRAW_OPTIONS,
    );
}

pub fn clear_background(draw_target: &mut DrawTarget) {
    draw_target.clear(COLOR_WHITE);
}
