use raqote::{AntialiasMode, BlendMode, DrawOptions, LineCap, LineJoin, SolidSource, StrokeStyle};

pub const CELL_SIZE: usize = 100;
pub const CELL_PADDING: usize = (0.15 * CELL_SIZE as f32) as usize;
pub const WIDTH: usize = 3 * CELL_SIZE + 2 * CELL_PADDING;
pub const HEIGHT: usize = 3 * CELL_SIZE + 2 * CELL_PADDING;

pub const LINE_STROKE_STYLE: StrokeStyle = StrokeStyle {
    width: CELL_SIZE as f32 / 20.0,
    cap: LineCap::Round,
    join: LineJoin::Round,
    miter_limit: 10.0,
    dash_array: vec![],
    dash_offset: 0.0,
};

pub const SYMBOL_STROKE_STYLE: StrokeStyle = StrokeStyle {
    width: CELL_SIZE as f32 * 0.2,
    cap: LineCap::Round,
    join: LineJoin::Round,
    miter_limit: 10.0,
    dash_array: vec![],
    dash_offset: 0.0,
};

pub const DRAW_OPTIONS: DrawOptions = DrawOptions {
    antialias: AntialiasMode::Gray,
    blend_mode: BlendMode::Src,
    alpha: 1.0,
};

pub const COLOR_WHITE: SolidSource = SolidSource {
    r: 0xEF,
    g: 0xE6,
    b: 0xDD,
    a: 0xFF,
};

pub const COLOR_BLUE: SolidSource = SolidSource {
    r: 0x23,
    g: 0x64,
    b: 0xAA,
    a: 0xFF,
};

pub const COLOR_RED: SolidSource = SolidSource {
    r: 0xBB,
    g: 0x44,
    b: 0x30,
    a: 0xFF,
};

pub const COLOR_GRAY: SolidSource = SolidSource {
    r: 0x4F,
    g: 0x5D,
    b: 0x75,
    a: 0xFF,
};

pub const COLOR_ORANGE: SolidSource = SolidSource {
    r: 0xFF,
    g: 0x9B,
    b: 0x71,
    a: 0xFF,
};
