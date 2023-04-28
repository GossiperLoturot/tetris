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

pub struct GameContext {
    pub blocks: Vec<Vec<Option<BlockColor>>>,
}

impl GameContext {
    pub fn new() -> Self {
        Self {
            blocks: vec![vec![None; 10]; 20],
        }
    }
}
