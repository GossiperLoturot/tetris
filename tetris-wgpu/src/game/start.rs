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

    pub fn input(&mut self, input: &winit::event::KeyboardInput, flow: &mut super::GameSystemFlow) {
        if let Some(virtual_keycode) = input.virtual_keycode {
            use winit::event::ElementState;
            use winit::event::VirtualKeyCode;
            match input.state {
                ElementState::Pressed if !self.pressed.contains(&virtual_keycode) => {
                    match virtual_keycode {
                        VirtualKeyCode::Return => {
                            *flow = super::GameSystemFlow::To(super::GameSystem::Playing(
                                super::playing::GameSystem::new(),
                            ))
                        }
                        _ => {}
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

    pub fn update(&mut self, flow: &mut super::GameSystemFlow) {
        // nothing
    }

    pub fn context(&self) -> GameContext {
        GameContext
    }
}
