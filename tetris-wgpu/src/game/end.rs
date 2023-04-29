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

    pub fn input(&mut self, input: &winit::event::KeyboardInput, flow: &mut super::GameSystemFlow) {
        // nothing
    }

    pub fn update(&mut self, flow: &mut super::GameSystemFlow) {
        // nothing
    }

    pub fn context(&self) -> GameContext {
        GameContext { score: &self.score }
    }
}
