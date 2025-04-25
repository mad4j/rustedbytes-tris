use raqote::{DrawTarget, PathBuilder, Source};

use crate::consts;

pub fn draw_grid(draw_target: &mut DrawTarget) {
    // Draw vertical lines

    for i in 1..3 {
        let offset = (consts::CELL_PADDING + i * consts::CELL_SIZE) as f32;

        // Vertical line
        let mut pb = PathBuilder::new();
        pb.move_to(offset, 0.0 + consts::CELL_PADDING as f32);
        pb.line_to(offset, consts::HEIGHT as f32 - consts::CELL_PADDING as f32);
        let path = pb.finish();
        draw_target.stroke(
            &path,
            &Source::Solid(consts::COLOR_GRAY),
            &consts::LINE_STROKE_STYLE,
            &consts::DRAW_OPTIONS,
        );

        // Horizontal line
        let mut pb = PathBuilder::new();
        pb.move_to(0.0 + consts::CELL_PADDING as f32, offset);
        pb.line_to(consts::WIDTH as f32 - consts::CELL_PADDING as f32, offset);
        let path = pb.finish();
        draw_target.stroke(
            &path,
            &Source::Solid(consts::COLOR_GRAY),
            &consts::LINE_STROKE_STYLE,
            &consts::DRAW_OPTIONS,
        );
    }
}

pub fn draw_player_x(draw_target: &mut DrawTarget, x: usize, y: usize) {
    let start_x = (consts::CELL_PADDING + x * consts::CELL_SIZE) as f32;
    let start_y = (consts::CELL_PADDING + y * consts::CELL_SIZE) as f32;
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
        &consts::DRAW_OPTIONS,
    );

    let mut pb = PathBuilder::new();
    pb.move_to(
        start_x + consts::CELL_SIZE as f32 - offset,
        start_y + offset,
    );
    pb.line_to(
        start_x + offset,
        start_y + consts::CELL_SIZE as f32 - offset,
    );
    let path = pb.finish();
    draw_target.stroke(
        &path,
        &Source::Solid(consts::COLOR_RED),
        &consts::SYMBOL_STROKE_STYLE,
        &consts::DRAW_OPTIONS,
    );
}

pub fn draw_player_o(draw_target: &mut DrawTarget, x: usize, y: usize) {
    let start_x = (consts::CELL_PADDING + x * consts::CELL_SIZE) as f32;
    let start_y = (consts::CELL_PADDING + y * consts::CELL_SIZE) as f32;

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
        &consts::DRAW_OPTIONS,
    );
}

pub fn draw_winning_line(draw_target: &mut DrawTarget, line: &[(usize, usize); 3]) {
    let start_x = (line[0].0 * consts::CELL_SIZE + consts::CELL_SIZE / 2) as f32;
    let start_y = (line[0].1 * consts::CELL_SIZE + consts::CELL_SIZE / 2) as f32;
    let end_x = (line[2].0 * consts::CELL_SIZE + consts::CELL_SIZE / 2) as f32;
    let end_y = (line[2].1 * consts::CELL_SIZE + consts::CELL_SIZE / 2) as f32;

    let mut pb = PathBuilder::new();
    pb.move_to(
        start_x + consts::CELL_PADDING as f32,
        start_y + consts::CELL_PADDING as f32,
    );
    pb.line_to(
        end_x + consts::CELL_PADDING as f32,
        end_y + consts::CELL_PADDING as f32,
    );
    let path = pb.finish();
    draw_target.stroke(
        &path,
        &Source::Solid(consts::COLOR_GREEN),
        &consts::LINE_STROKE_STYLE,
        &consts::DRAW_OPTIONS,
    );
}

pub fn clear_backgorund(draw_target: &mut DrawTarget) {
    draw_target.clear(consts::COLOR_WHITE);
}
