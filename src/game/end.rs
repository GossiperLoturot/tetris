use crate::game;

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

    pub fn input(
        &mut self,
        _input: &winit::event::KeyboardInput,
        _flow: &mut game::GameSystemFlow,
    ) {
        // nothing
    }

    pub fn update(&mut self, _flow: &mut game::GameSystemFlow) {
        // nothing
    }

    pub fn context(&self) -> GameContext {
        GameContext { score: &self.score }
    }
}
