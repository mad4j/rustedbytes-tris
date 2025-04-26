/*!
 * This module contains all the constants used in the game.
 * It includes the size of the cells, colors, stroke styles, and drawing options.
 * These constants are used to define the appearance of the game board and symbols.
 */

use raqote::{AntialiasMode, BlendMode, DrawOptions, LineCap, LineJoin, SolidSource, StrokeStyle};


/// The size (in pixels) of each cell in the game board.
/// The cell size is used to calculate the dimensions of the game board and the symbols drawn on it.
pub const CELL_SIZE: usize = 100;


/// The padding (in pixels) around each cell.
pub const CELL_PADDING: usize = (0.15 * CELL_SIZE as f32) as usize;


/// The width and height of the game board.
/// The game board is a square grid of cells, so the width and height are equal.
pub const WIDTH: usize = 3 * CELL_SIZE + 2 * CELL_PADDING;
pub const HEIGHT: usize = 3 * CELL_SIZE + 2 * CELL_PADDING;


/// The stroke style for the lines drawn on the game board.
pub const GRID_LINE_STROKE_STYLE: StrokeStyle = StrokeStyle {
    width: CELL_SIZE as f32 / 20.0,
    cap: LineCap::Round,
    join: LineJoin::Round,
    miter_limit: 10.0,
    dash_array: vec![],
    dash_offset: 0.0,
};


/// The stroke style for the lines drawn on the game board.
pub const WIN_LINE_STROKE_STYLE: StrokeStyle = StrokeStyle {
    width: CELL_SIZE as f32 / 10.0,
    cap: LineCap::Round,
    join: LineJoin::Round,
    miter_limit: 10.0,
    dash_array: vec![],
    dash_offset: 0.0,
};


/// The stroke style for the symbols (X and O) drawn on the game board.
pub const SYMBOL_STROKE_STYLE: StrokeStyle = StrokeStyle {
    width: CELL_SIZE as f32 * 0.2,
    cap: LineCap::Round,
    join: LineJoin::Round,
    miter_limit: 10.0,
    dash_array: vec![],
    dash_offset: 0.0,
};


/// The drawing options used for rendering the game board and symbols.
/// The drawing options include antialiasing mode, blend mode, and alpha transparency.
pub const DRAW_OPTIONS: DrawOptions = DrawOptions {
    antialias: AntialiasMode::Gray,
    blend_mode: BlendMode::Src,
    alpha: 1.0,
};


/// The color white used for the background and grid lines.
pub const COLOR_WHITE: SolidSource = SolidSource {
    r: 0xEF,
    g: 0xE6,
    b: 0xDD,
    a: 0xFF,
};


/// The color blue used for the winning line
pub const COLOR_BLUE: SolidSource = SolidSource {
    r: 0x23,
    g: 0x64,
    b: 0xAA,
    a: 0xFF,
};

/// The color red used for the X symbol.
pub const COLOR_RED: SolidSource = SolidSource {
    r: 0xBB,
    g: 0x44,
    b: 0x30,
    a: 0xFF,
};

/// The color gray used for the grid lines.
pub const COLOR_GRAY: SolidSource = SolidSource {
    r: 0x4F,
    g: 0x5D,
    b: 0x75,
    a: 0xFF,
};

/// The color orange used for the O symbol.
pub const COLOR_ORANGE: SolidSource = SolidSource {
    r: 0xFF,
    g: 0x9B,
    b: 0x71,
    a: 0xFF,
};
