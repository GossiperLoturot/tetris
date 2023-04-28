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

pub struct GameData {
    pub blocks: Vec<Vec<Option<BlockColor>>>,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            blocks: vec![vec![None; 10]; 20],
        }
    }
}
