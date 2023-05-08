pub const WIDTH: f32 = 10.0;
pub const HEIGHT: f32 = 20.0;
pub const TEXT_SCALE: f32 = 16.0;

#[rustfmt::skip]
pub mod color {
    pub const BG_DEFAULT:   [f32; 3] = [1.000, 1.000, 1.000];
    pub const BG_SUBTLE:    [f32; 3] = [0.921, 0.938, 0.955];
    pub const FG_CYAN:      [f32; 3] = [0.000, 0.666, 1.000];
    pub const FG_YELLOW:    [f32; 3] = [1.000, 0.666, 0.000];
    pub const FG_GREEN:     [f32; 3] = [0.133, 1.000, 0.000];
    pub const FG_RED:       [f32; 3] = [1.000, 0.000, 0.133];
    pub const FG_BLUE:      [f32; 3] = [0.000, 0.133, 1.000];
    pub const FG_ORANGE:    [f32; 3] = [1.000, 0.133, 0.000];
    pub const FG_PURPLE:    [f32; 3] = [0.133, 0.000, 1.000];
    pub const TEXT:         [f32; 4] = [0.000, 0.000, 0.000, 1.000];
    pub const TEXT_PLAYING: [f32; 4] = [0.000, 0.000, 0.000, 0.800];
}
