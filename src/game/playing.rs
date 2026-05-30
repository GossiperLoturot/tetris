use crate::game;
use std::collections::HashSet;

pub const MAX_BLOCK_WIDTH: i32 = 10;
pub const MAX_BLOCK_HEIGHT: i32 = 25;
pub const SPAWN_BLOCK_X: i32 = 3;
pub const SPAWN_BLOCK_Y: i32 = 20;
pub const MAX_STACK_HEIGHT: i32 = 20;

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

#[derive(Clone)]
pub struct BlockSetTemplate {
    pub content: &'static [(i32, i32)],
    pub rotation_origin: (f32, f32),
    pub color: BlockColor,
}

#[rustfmt::skip]
pub const BLOCK_SET_TEMPLATES: &[BlockSetTemplate] = &[
    BlockSetTemplate { content: &[(0, 0), (1, 0), (2, 0), (3, 0)], rotation_origin: (1.5, 0.5), color: BlockColor::Cyan },    // I tetromino
    BlockSetTemplate { content: &[(1, 0), (2, 0), (2, 1), (1, 1)], rotation_origin: (1.5, 0.5), color: BlockColor::Yellow },  // O tetromino
    BlockSetTemplate { content: &[(0, 0), (1, 0), (1, 1), (2, 1)], rotation_origin: (1.0, 0.0), color: BlockColor::Green },   // S tetromino
    BlockSetTemplate { content: &[(0, 1), (1, 1), (1, 0), (2, 0)], rotation_origin: (1.0, 0.0), color: BlockColor::Red },     // Z tetromino
    BlockSetTemplate { content: &[(0, 1), (0, 0), (1, 0), (2, 0)], rotation_origin: (1.0, 0.0), color: BlockColor::Blue },    // J tetromino
    BlockSetTemplate { content: &[(0, 0), (1, 0), (2, 0), (2, 1)], rotation_origin: (1.0, 0.0), color: BlockColor::Orange },  // L tetromino
    BlockSetTemplate { content: &[(0, 0), (1, 0), (2, 0), (1, 1)], rotation_origin: (1.0, 0.0), color: BlockColor::Purple },  // T tetromino
];

#[derive(Clone)]
pub struct BlockSet {
    pub x: i32,
    pub y: i32,
    pub content: Vec<(i32, i32)>,
    pub template: BlockSetTemplate,
}

pub struct GameContext<'a> {
    pub blocks: &'a Vec<Vec<Option<BlockColor>>>,
    pub block_set: &'a Option<BlockSet>,
    pub score: &'a i32,
    pub paused: &'a bool,
}

pub struct GameSystem {
    rng: rand::rngs::ThreadRng,
    blocks: Vec<Vec<Option<BlockColor>>>,
    block_set: Option<BlockSet>,
    pressed: HashSet<winit::event::VirtualKeyCode>,
    last_update: Option<std::time::Instant>,
    update_interval: std::time::Duration,
    score: i32,
    paused: bool,
}

impl GameSystem {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            blocks: vec![vec![None; MAX_BLOCK_WIDTH as usize]; MAX_BLOCK_HEIGHT as usize],
            block_set: None,
            pressed: HashSet::new(),
            last_update: None,
            update_interval: std::time::Duration::from_millis(400),
            score: 0,
            paused: false,
        }
    }

    pub fn input(&mut self, input: &winit::event::KeyboardInput, flow: &mut game::GameSystemFlow) {
        if let Some(virtual_keycode) = input.virtual_keycode {
            use winit::event::ElementState;
            use winit::event::VirtualKeyCode;
            match input.state {
                ElementState::Pressed if !self.pressed.contains(&virtual_keycode) => {
                    match virtual_keycode {
                        VirtualKeyCode::P => {
                            self.paused = !self.paused;

                            if !self.paused {
                                self.last_update = Some(std::time::Instant::now());
                            }
                        }
                        VirtualKeyCode::Space if !self.paused => {
                            self.hard_drop_block_set();

                            if self.block_set.is_none() {
                                self.spawn_block_set();
                            }
                            self.check_and_end(flow);
                        }
                        VirtualKeyCode::Up | VirtualKeyCode::X if !self.paused => {
                            self.rotate_block_set(true);
                        }
                        VirtualKeyCode::Z if !self.paused => {
                            self.rotate_block_set(false);
                        }
                        VirtualKeyCode::Down if !self.paused => {
                            self.down_block_set();
                        }
                        VirtualKeyCode::Right if !self.paused => {
                            self.right_block_set();
                        }
                        VirtualKeyCode::Left if !self.paused => {
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

    pub fn update(&mut self, flow: &mut game::GameSystemFlow) {
        if self.paused {
            return;
        }

        if self
            .last_update
            .map(|last_update| self.update_interval < last_update.elapsed())
            .unwrap_or(true)
        {
            self.place_block_set();
            self.down_block_set();

            if self.block_set.is_none() {
                self.spawn_block_set();
            }
            self.check_and_end(flow);

            self.last_update = Some(std::time::Instant::now());
        }
    }

    fn is_valid_placement(&self, block_set: &BlockSet) -> bool {
        block_set.content.iter().all(|(x, y)| {
            let x = block_set.x + x;
            let y = block_set.y + y;
            (0..MAX_BLOCK_WIDTH).contains(&x)
                && (0..MAX_BLOCK_HEIGHT).contains(&y)
                && self.blocks[y as usize][x as usize].is_none()
        })
    }

    fn is_game_over(&self) -> bool {
        let placed_blocks_over_height = self
            .blocks
            .iter()
            .enumerate()
            .skip(MAX_STACK_HEIGHT as usize)
            .any(|(_, line)| line.iter().any(|block| block.is_some()));
        let spawned_block_collided = self
            .block_set
            .as_ref()
            .map(|block_set| !self.is_valid_placement(block_set))
            .unwrap_or(false);

        placed_blocks_over_height || spawned_block_collided
    }

    fn check_and_end(&self, flow: &mut game::GameSystemFlow) {
        if self.is_game_over() {
            let state = game::GameSystem::End(game::end::GameSystem::new(self.score));
            *flow = game::GameSystemFlow::To(state);
        }
    }

    fn spawn_block_set(&mut self) {
        use rand::seq::SliceRandom;

        let block_set_template = BLOCK_SET_TEMPLATES.choose(&mut self.rng).unwrap();

        self.block_set = Some(BlockSet {
            x: SPAWN_BLOCK_X,
            y: SPAWN_BLOCK_Y,
            content: block_set_template.content.to_vec(),
            template: block_set_template.clone(),
        });
    }

    fn place_block_set(&mut self) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut next = block_set.clone();

            next.y -= 1;

            if self.is_valid_placement(block_set) && !self.is_valid_placement(&next) {
                for (x, y) in block_set.content.iter() {
                    let x = block_set.x + *x;
                    let y = block_set.y + *y;
                    self.blocks[y as usize][x as usize] = Some(block_set.template.color.clone());
                }
                self.block_set = None;

                self.erase_block_line();
            }
        }
    }

    fn rotate_block_set(&mut self, clockwise: bool) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut next = block_set.clone();

            for (x, y) in next.content.iter_mut() {
                let (origin_x, origin_y) = block_set.template.rotation_origin;
                let shift_x = *x as f32 - origin_x;
                let shift_y = *y as f32 - origin_y;
                let (rotated_x, rotated_y) = if clockwise {
                    (shift_y, -shift_x)
                } else {
                    (-shift_y, shift_x)
                };

                *x = (origin_x + rotated_x).round() as i32;
                *y = (origin_y + rotated_y).round() as i32;
            }

            if self.is_valid_placement(&next) {
                self.block_set = Some(next);
            }
        }
    }

    fn down_block_set(&mut self) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut next = block_set.clone();

            next.y -= 1;

            if self.is_valid_placement(&next) {
                self.block_set = Some(next);
            }
        }
    }

    fn hard_drop_block_set(&mut self) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut dropped = block_set.clone();
            loop {
                let mut next = dropped.clone();

                next.y -= 1;

                if self.is_valid_placement(&next) {
                    dropped = next;
                } else {
                    break;
                }
            }
            self.block_set = Some(dropped);
            self.place_block_set();
        }
    }

    fn right_block_set(&mut self) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut next = block_set.clone();

            next.x += 1;

            if self.is_valid_placement(&next) {
                self.block_set = Some(next);
            }
        }
    }

    fn left_block_set(&mut self) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut next = block_set.clone();

            next.x -= 1;

            if self.is_valid_placement(&next) {
                self.block_set = Some(next);
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

        for row in 0..MAX_BLOCK_HEIGHT as usize {
            let down = row_nums
                .iter()
                .filter(|erased_row| **erased_row < row)
                .count();
            self.blocks.swap(row, row - down);
        }
    }

    pub fn context(&'_ self) -> GameContext<'_> {
        GameContext {
            blocks: &self.blocks,
            block_set: &self.block_set,
            score: &self.score,
            paused: &self.paused,
        }
    }
}
