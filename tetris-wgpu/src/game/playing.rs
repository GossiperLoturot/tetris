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

const BLOCK_SET_CONTENTS: &[&[(i32, i32, BlockColor)]] = &[
    &[
        (-1, 0, BlockColor::Cyan),
        (0, 0, BlockColor::Cyan),
        (1, 0, BlockColor::Cyan),
        (2, 0, BlockColor::Cyan),
    ],
    &[
        (0, 0, BlockColor::Yellow),
        (1, 0, BlockColor::Yellow),
        (1, 1, BlockColor::Yellow),
        (0, 1, BlockColor::Yellow),
    ],
    &[
        (-1, -1, BlockColor::Green),
        (0, -1, BlockColor::Green),
        (0, 0, BlockColor::Green),
        (1, 0, BlockColor::Green),
    ],
    &[
        (-1, 0, BlockColor::Red),
        (0, 0, BlockColor::Red),
        (0, -1, BlockColor::Red),
        (1, -1, BlockColor::Red),
    ],
    &[
        (-1, 0, BlockColor::Blue),
        (-1, -1, BlockColor::Blue),
        (0, -1, BlockColor::Blue),
        (1, -1, BlockColor::Blue),
    ],
    &[
        (-1, -1, BlockColor::Orange),
        (0, -1, BlockColor::Orange),
        (1, -1, BlockColor::Orange),
        (1, 0, BlockColor::Orange),
    ],
    &[
        (-1, -1, BlockColor::Purple),
        (0, -1, BlockColor::Purple),
        (1, -1, BlockColor::Purple),
        (0, 0, BlockColor::Purple),
    ],
];

#[derive(Clone)]
pub struct BlockSet {
    pub x: i32,
    pub y: i32,
    pub content: Vec<(i32, i32, BlockColor)>,
}

pub struct GameContext<'a> {
    pub blocks: &'a Vec<Vec<Option<BlockColor>>>,
    pub block_set: &'a Option<BlockSet>,
    pub score: &'a i32,
}

pub struct GameSystem {
    block_width: u32,
    block_height: u32,
    spawn_block_x: i32,
    spawn_block_y: i32,
    rng: rand::rngs::ThreadRng,
    blocks: Vec<Vec<Option<BlockColor>>>,
    block_set: Option<BlockSet>,
    pressed: HashSet<winit::event::VirtualKeyCode>,
    last_update: Option<std::time::Instant>,
    update_interval: std::time::Duration,
    score: i32,
}

impl GameSystem {
    pub fn new() -> Self {
        let block_width = 10;
        let block_height = 25;

        Self {
            block_width,
            block_height,
            spawn_block_x: 5,
            spawn_block_y: 20,
            rng: rand::thread_rng(),
            blocks: vec![vec![None; block_width as usize]; block_height as usize],
            block_set: None,
            pressed: HashSet::new(),
            last_update: None,
            update_interval: std::time::Duration::from_millis(400),
            score: 0,
        }
    }

    pub fn input(&mut self, input: &winit::event::KeyboardInput, flow: &mut super::GameSystemFlow) {
        if let Some(virtual_keycode) = input.virtual_keycode {
            use winit::event::ElementState;
            use winit::event::VirtualKeyCode;
            match input.state {
                ElementState::Pressed if !self.pressed.contains(&virtual_keycode) => {
                    match virtual_keycode {
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

    pub fn update(&mut self, flow: &mut super::GameSystemFlow) {
        if self
            .last_update
            .map(|last_update| self.update_interval < last_update.elapsed())
            .unwrap_or(true)
        {
            self.place_block_set();
            self.down_block_set();

            if self.block_set.is_none() {
                self.spawn_block_set();

                if !self.is_valid_placement(self.block_set.as_ref().unwrap()) {
                    *flow = super::GameSystemFlow::To(super::GameSystem::End(
                        super::end::GameSystem::new(self.score),
                    ))
                }
            }

            self.last_update = Some(std::time::Instant::now());
        }
    }

    fn is_valid_placement(&self, block_set: &BlockSet) -> bool {
        block_set.content.iter().all(|(x, y, _)| {
            let x = block_set.x + x;
            let y = block_set.y + y;
            0 <= x
                && x < self.block_width as i32
                && 0 <= y
                && y < self.block_height as i32
                && self.blocks[y as usize][x as usize].is_none()
        })
    }

    fn spawn_block_set(&mut self) {
        use rand::seq::SliceRandom;

        let content = BLOCK_SET_CONTENTS.choose(&mut self.rng).unwrap().to_vec();

        self.block_set = Some(BlockSet {
            x: self.spawn_block_x,
            y: self.spawn_block_y,
            content,
        });
    }

    fn place_block_set(&mut self) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut cloned = block_set.clone();

            cloned.y -= 1;

            if self.is_valid_placement(block_set) && !self.is_valid_placement(&cloned) {
                for (x, y, block_color) in block_set.content.iter() {
                    let x = block_set.x + *x;
                    let y = block_set.y + *y;
                    self.blocks[y as usize][x as usize] = Some(block_color.clone());
                }
                self.block_set = None;

                self.erase_block_line();
            }
        }
    }

    fn rotate_block_set(&mut self) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut cloned = block_set.clone();

            for (x, y, _) in cloned.content.iter_mut() {
                (*x, *y) = (*y, -*x);
            }

            if self.is_valid_placement(&cloned) {
                self.block_set = Some(cloned);
            }
        }
    }

    fn down_block_set(&mut self) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut cloned = block_set.clone();

            cloned.y -= 1;

            if self.is_valid_placement(&cloned) {
                self.block_set = Some(cloned);
            }
        }
    }

    fn right_block_set(&mut self) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut cloned = block_set.clone();

            cloned.x += 1;

            if self.is_valid_placement(&cloned) {
                self.block_set = Some(cloned);
            }
        }
    }

    fn left_block_set(&mut self) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut cloned = block_set.clone();

            cloned.x -= 1;

            if self.is_valid_placement(&cloned) {
                self.block_set = Some(cloned);
            }
        }
    }

    fn erase_block_line(&mut self) {
        let mut row_nums = vec![];

        for (row, line) in self.blocks.iter_mut().enumerate() {
            let fill_line = line.iter().all(|block| block.is_some());
            if fill_line {
                line.iter_mut().for_each(|block| *block = None);
                row_nums.push(row);

                self.score += 1;
            }
        }

        for row in 0..self.block_height as usize {
            let down = row_nums
                .iter()
                .filter(|erased_row| **erased_row < row)
                .count();
            self.blocks.swap(row, row - down);
        }
    }

    pub fn context(&self) -> GameContext {
        GameContext {
            blocks: &self.blocks,
            block_set: &self.block_set,
            score: &self.score,
        }
    }
}
