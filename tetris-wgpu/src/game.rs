use std::collections::HashSet;

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

pub struct BlockSet {
    pub x: i32,
    pub y: i32,
    pub content: Vec<(i32, i32, BlockColor)>,
}

pub struct GameContext {
    pub blocks: Vec<Vec<Option<BlockColor>>>,
    pub block_set: Option<BlockSet>,
}

pub struct InputSystem {
    pressed: HashSet<winit::event::VirtualKeyCode>,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            pressed: HashSet::new(),
        }
    }

    pub fn update(
        &mut self,
        cx: &mut GameContext,
        state: &winit::event::ElementState,
        virtual_keycode: &winit::event::VirtualKeyCode,
    ) {
        use winit::event::ElementState;
        use winit::event::VirtualKeyCode;
        match state {
            ElementState::Pressed if !self.pressed.contains(virtual_keycode) => {
                match virtual_keycode {
                    VirtualKeyCode::Space => {
                        cx.block_set = Some(BlockSet {
                            x: 5,
                            y: 20,
                            content: vec![
                                (0, -2, BlockColor::Red),
                                (0, -1, BlockColor::Red),
                                (0, 0, BlockColor::Red),
                                (0, 1, BlockColor::Red),
                            ],
                        });
                    }
                    VirtualKeyCode::Return => {
                        cx.block_set = None;
                    }
                    VirtualKeyCode::Up => {
                        todo!();
                    }
                    VirtualKeyCode::Down => {
                        cx.block_set.as_mut().map(|block_set| block_set.y -= 1);
                    }
                    VirtualKeyCode::Right => {
                        cx.block_set.as_mut().map(|block_set| block_set.x += 1);
                    }
                    VirtualKeyCode::Left => {
                        cx.block_set.as_mut().map(|block_set| block_set.x -= 1);
                    }
                    _ => {}
                }
                self.pressed.insert(*virtual_keycode);
            }
            ElementState::Released => {
                self.pressed.remove(virtual_keycode);
            }
            _ => {}
        }
    }
}
