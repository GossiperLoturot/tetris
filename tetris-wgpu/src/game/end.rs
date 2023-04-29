pub struct GameContext<'a> {
    pub score: &'a i32,
}

pub struct GameSystem {
    score: i32,
}

impl GameSystem {
    pub fn new(score: i32) -> Self {
        Self { score }
    }

    pub fn context(&self) -> GameContext {
        GameContext { score: &self.score }
    }
}
