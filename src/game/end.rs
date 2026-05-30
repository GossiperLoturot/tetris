use std::collections::HashSet;

use crate::{consts, game};

pub struct GameContext<'a> {
    pub blocks: &'a Vec<Vec<Option<consts::BlockColor>>>,
    pub score: &'a i32,
}

pub struct GameSystem {
    pressed: HashSet<winit::keyboard::KeyCode>,

    blocks: Vec<Vec<Option<consts::BlockColor>>>,
    score: i32,
}

impl GameSystem {
    pub fn new(blocks: Vec<Vec<Option<consts::BlockColor>>>, score: i32) -> Self {
        Self {
            pressed: HashSet::new(),

            blocks,
            score,
        }
    }

    pub fn input(&mut self, input: &winit::event::KeyEvent, flow: &mut game::GameSystemFlow) {
        use winit::event::ElementState;
        use winit::keyboard::KeyCode;

        if let winit::keyboard::PhysicalKey::Code(code) = input.physical_key {
            match input.state {
                ElementState::Pressed if !self.pressed.contains(&code) => {
                    if code == KeyCode::Enter {
                        let state = game::GameSystem::Playing(game::playing::GameSystem::new());
                        *flow = game::GameSystemFlow::To(state);
                    }
                    self.pressed.insert(code);
                }
                ElementState::Released => {
                    self.pressed.remove(&code);
                }
                _ => {}
            }
        }
    }

    pub fn update(&mut self, _flow: &mut game::GameSystemFlow) {
        // nothing
    }

    pub fn context(&'_ self) -> GameContext<'_> {
        GameContext {
            blocks: &self.blocks,
            score: &self.score,
        }
    }
}
