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

pub struct GameContext<'a> {
    pub blocks: &'a Vec<Vec<Option<BlockColor>>>,
    pub block_set: &'a Option<BlockSet>,
}

pub struct GameSystem {
    block_width: u32,
    block_height: u32,
    spawn_block_x: i32,
    spawn_block_y: i32,
    blocks: Vec<Vec<Option<BlockColor>>>,
    block_set: Option<BlockSet>,
    pressed: HashSet<winit::event::VirtualKeyCode>,
}

impl GameSystem {
    pub fn new(
        block_width: u32,
        block_height: u32,
        spawn_block_x: i32,
        spawn_block_y: i32,
    ) -> Self {
        Self {
            block_width,
            block_height,
            spawn_block_x,
            spawn_block_y,
            blocks: vec![vec![None; block_width as usize]; block_height as usize],
            block_set: None,
            pressed: HashSet::new(),
        }
    }

    pub fn input(&mut self, input: &winit::event::KeyboardInput) {
        if let Some(virtual_keycode) = input.virtual_keycode {
            use winit::event::ElementState;
            use winit::event::VirtualKeyCode;
            match input.state {
                ElementState::Pressed if !self.pressed.contains(&virtual_keycode) => {
                    match virtual_keycode {
                        VirtualKeyCode::Space => {
                            self.spawn_block_set();
                        }
                        VirtualKeyCode::Return => {
                            self.place_block_set();
                        }
                        VirtualKeyCode::Up => {
                            self.rotate_block_set();
                        }
                        VirtualKeyCode::Down => {
                            self.down_block_set();
                        }
                        VirtualKeyCode::Right => {
                            self.right_block_set();
                        }
                        VirtualKeyCode::Left => {
                            self.left_block_set();
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

    fn spawn_block_set(&mut self) {
        self.block_set = Some(BlockSet {
            x: self.spawn_block_x,
            y: self.spawn_block_y,
            content: vec![
                (0, -1, BlockColor::Red),
                (0, 0, BlockColor::Red),
                (0, 1, BlockColor::Red),
                (0, 2, BlockColor::Red),
            ],
        });
    }

    fn place_block_set(&mut self) {
        self.block_set = None;
    }

    fn rotate_block_set(&mut self) {
        self.block_set.as_mut().map(|block_set| {
            block_set
                .content
                .iter_mut()
                .for_each(|(x, y, _)| (*x, *y) = (*y, -*x))
        });
    }

    fn down_block_set(&mut self) {
        self.block_set.as_mut().map(|block_set| block_set.y -= 1);
    }

    fn right_block_set(&mut self) {
        self.block_set.as_mut().map(|block_set| block_set.x += 1);
    }

    fn left_block_set(&mut self) {
        self.block_set.as_mut().map(|block_set| block_set.x -= 1);
    }

    pub fn context(&self) -> GameContext {
        GameContext {
            blocks: &self.blocks,
            block_set: &self.block_set,
        }
    }
}
