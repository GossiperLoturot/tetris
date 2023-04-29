use std::collections::HashSet;

pub struct GameSystem {
    pressed: HashSet<winit::event::VirtualKeyCode>,
}

impl GameSystem {
    pub fn new() -> Self {
        Self {
            pressed: HashSet::new(),
        }
    }

    pub fn input(
        &mut self,
        input: &winit::event::KeyboardInput,
        event_sender: &winit::event_loop::EventLoopProxy<super::GameEvent>,
    ) {
        if let Some(virtual_keycode) = input.virtual_keycode {
            use winit::event::ElementState;
            use winit::event::VirtualKeyCode;
            match input.state {
                ElementState::Pressed if !self.pressed.contains(&virtual_keycode) => {
                    match virtual_keycode {
                        VirtualKeyCode::Return => {
                            event_sender.send_event(super::GameEvent::Play).unwrap();
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
}
