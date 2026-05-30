use std::collections::HashSet;

use crate::game;

pub struct GameContext;

pub struct GameSystem {
    pressed: HashSet<winit::keyboard::KeyCode>,
}

impl GameSystem {
    pub fn new() -> Self {
        Self {
            pressed: HashSet::new(),
        }
    }

    pub fn input(&mut self, input: &winit::event::KeyEvent, flow: &mut game::GameSystemFlow) {
        use winit::event::ElementState;
        use winit::keyboard::KeyCode;

        if let winit::keyboard::PhysicalKey::Code(virtual_keycode) = input.physical_key {
            match input.state {
                ElementState::Pressed if !self.pressed.contains(&virtual_keycode) => {
                    if virtual_keycode == KeyCode::Enter {
                        let state = game::GameSystem::Playing(game::playing::GameSystem::new());
                        *flow = game::GameSystemFlow::To(state);
                    }
                    self.pressed.insert(virtual_keycode);
                }
                ElementState::Released => {
                    self.pressed.remove(&virtual_keycode);
                }
                _ => {}
            }
        }
    }

    pub fn update(&mut self, _flow: &mut game::GameSystemFlow) {
        // nothing
    }

    pub fn context(&self) -> GameContext {
        GameContext
    }
}
