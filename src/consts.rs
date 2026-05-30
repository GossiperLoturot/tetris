use std::time::Duration;

pub const MAX_BLOCK_WIDTH: i32 = 10;
pub const MAX_BLOCK_HEIGHT: i32 = 25;
pub const MAX_STACK_HEIGHT: i32 = 20;

pub const SPAWN_BLOCK_X: i32 = 3;
pub const SPAWN_BLOCK_Y: i32 = 20;

pub const UPDATE_INTERVAL: Duration = Duration::from_millis(400);

#[derive(Clone)]
pub enum BlockColor {
    Cyan,
    Yellow,
    Green,
    Red,
    Blue,
    Orange,
    Purple,
}

#[derive(Clone)]
pub struct MinoTemplate {
    pub blocks: &'static [(i32, i32)],
    pub rotation_origin: (f32, f32),
    pub color: BlockColor,
}

#[rustfmt::skip]
pub const MINO_TEMPLATES: &[MinoTemplate] = &[
    MinoTemplate { blocks: &[(0, 0), (1, 0), (2, 0), (3, 0)], rotation_origin: (1.5, 0.5), color: BlockColor::Cyan },    // I tetromino
    MinoTemplate { blocks: &[(1, 0), (2, 0), (2, 1), (1, 1)], rotation_origin: (1.5, 0.5), color: BlockColor::Yellow },  // O tetromino
    MinoTemplate { blocks: &[(0, 0), (1, 0), (1, 1), (2, 1)], rotation_origin: (1.0, 0.0), color: BlockColor::Green },   // S tetromino
    MinoTemplate { blocks: &[(0, 1), (1, 1), (1, 0), (2, 0)], rotation_origin: (1.0, 0.0), color: BlockColor::Red },     // Z tetromino
    MinoTemplate { blocks: &[(0, 1), (0, 0), (1, 0), (2, 0)], rotation_origin: (1.0, 0.0), color: BlockColor::Blue },    // J tetromino
    MinoTemplate { blocks: &[(0, 0), (1, 0), (2, 0), (2, 1)], rotation_origin: (1.0, 0.0), color: BlockColor::Orange },  // L tetromino
    MinoTemplate { blocks: &[(0, 0), (1, 0), (2, 0), (1, 1)], rotation_origin: (1.0, 0.0), color: BlockColor::Purple },  // T tetromino
];

pub const VIEW_WIDTH: f32 = 10.0;
pub const VIEW_HEIGHT: f32 = 22.0;
pub const TEXT_SCALE: f32 = 16.0;

#[rustfmt::skip]
pub mod block_color {
    pub const BG_DEFAULT:   [f32; 3] = [1.000, 1.000, 1.000];
    pub const BG_SUBTLE:    [f32; 3] = [0.921, 0.938, 0.955];
    pub const BG_MAX_STACK: [f32; 3] = [1.000, 0.750, 0.750];
    pub const FG_CYAN:      [f32; 3] = [0.000, 0.666, 1.000];
    pub const FG_YELLOW:    [f32; 3] = [1.000, 0.666, 0.000];
    pub const FG_GREEN:     [f32; 3] = [0.133, 1.000, 0.000];
    pub const FG_RED:       [f32; 3] = [1.000, 0.000, 0.133];
    pub const FG_BLUE:      [f32; 3] = [0.000, 0.133, 1.000];
    pub const FG_ORANGE:    [f32; 3] = [1.000, 0.133, 0.000];
    pub const FG_PURPLE:    [f32; 3] = [0.133, 0.000, 1.000];
}

#[rustfmt::skip]
pub mod text_color {
    pub const TEXT_PRIMARY:   [f32; 4] = [0.000, 0.000, 0.000, 1.000];
    pub const TEXT_SECONDARY: [f32; 4] = [0.000, 0.000, 0.000, 0.800];
}

pub fn to_rgb(value: &BlockColor) -> [f32; 3] {
    match value {
        BlockColor::Cyan => block_color::FG_CYAN,
        BlockColor::Yellow => block_color::FG_YELLOW,
        BlockColor::Green => block_color::FG_GREEN,
        BlockColor::Red => block_color::FG_RED,
        BlockColor::Blue => block_color::FG_BLUE,
        BlockColor::Orange => block_color::FG_ORANGE,
        BlockColor::Purple => block_color::FG_PURPLE,
    }
}
