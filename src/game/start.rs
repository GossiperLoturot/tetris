use crate::game;
use std::collections::HashSet;

pub struct GameContext;

pub struct GameSystem {
    pressed: HashSet<winit::event::VirtualKeyCode>,
}

impl GameSystem {
    pub fn new() -> Self {
        Self {
            pressed: HashSet::new(),
        }
    }

    pub fn input(&mut self, input: &winit::event::KeyboardInput, flow: &mut game::GameSystemFlow) {
        if let Some(virtual_keycode) = input.virtual_keycode {
            use winit::event::ElementState;
            use winit::event::VirtualKeyCode;
            match input.state {
                ElementState::Pressed if !self.pressed.contains(&virtual_keycode) => {
                    if virtual_keycode == VirtualKeyCode::Return {
                        *flow = game::GameSystemFlow::To(game::GameSystem::Playing(
                            game::playing::GameSystem::new(),
                        ))
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
