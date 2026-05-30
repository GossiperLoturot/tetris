use crate::game;
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

#[rustfmt::skip]
const BLOCK_SET_TABLE: &[(&[(i32, i32)], (f32, f32), BlockColor)] = &[
    (&[(-1,  0), ( 0,  0), (1,  0), (2,  0)], (0.5, 0.5), BlockColor::Cyan),
    (&[( 0,  0), ( 1,  0), (1,  1), (0,  1)], (0.5, 0.5), BlockColor::Yellow),
    (&[(-1, -1), ( 0, -1), (0,  0), (1,  0)], (0.0, 0.0), BlockColor::Green),
    (&[(-1,  0), ( 0,  0), (0, -1), (1, -1)], (0.0, 0.0), BlockColor::Red),
    (&[(-1,  0), (-1, -1), (0, -1), (1, -1)], (0.0, 0.0), BlockColor::Blue),
    (&[(-1, -1), ( 0, -1), (1, -1), (1,  0)], (0.0, 0.0), BlockColor::Orange),
    (&[(-1, -1), ( 0, -1), (1, -1), (0,  0)], (0.0, 0.0), BlockColor::Purple),
];

#[derive(Clone)]
pub struct BlockSet {
    pub x: i32,
    pub y: i32,
    pub rotation_origin: (f32, f32),
    pub color: BlockColor,
    pub content: Vec<(i32, i32)>,
}

pub const MAX_STACK_HEIGHT: i32 = 20;

pub struct GameContext<'a> {
    pub blocks: &'a Vec<Vec<Option<BlockColor>>>,
    pub block_set: &'a Option<BlockSet>,
    pub score: &'a i32,
    pub paused: &'a bool,
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
    paused: bool,
}

impl GameSystem {
    pub fn new() -> Self {
        let block_width = 10;
        let block_height = 25;

        Self {
            block_width,
            block_height,
            spawn_block_x: 4,
            spawn_block_y: 22,
            rng: rand::thread_rng(),
            blocks: vec![vec![None; block_width as usize]; block_height as usize],
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
            0 <= x
                && x < self.block_width as i32
                && 0 <= y
                && y < self.block_height as i32
                && self.blocks[y as usize][x as usize].is_none()
        })
    }

    fn check_and_end(&self, flow: &mut game::GameSystemFlow) {
        if self.is_game_over() {
            *flow = game::GameSystemFlow::To(game::GameSystem::End(game::end::GameSystem::new(
                self.score,
            )));
        }
    }

    fn spawn_block_set(&mut self) {
        use rand::seq::SliceRandom;

        let (content, rotation_origin, color) = BLOCK_SET_TABLE.choose(&mut self.rng).unwrap();

        self.block_set = Some(BlockSet {
            x: self.spawn_block_x,
            y: self.spawn_block_y,
            rotation_origin: *rotation_origin,
            color: color.clone(),
            content: content.to_vec(),
        });
    }

    fn place_block_set(&mut self) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut cloned = block_set.clone();

            cloned.y -= 1;

            if self.is_valid_placement(block_set) && !self.is_valid_placement(&cloned) {
                for (x, y) in block_set.content.iter() {
                    let x = block_set.x + *x;
                    let y = block_set.y + *y;
                    self.blocks[y as usize][x as usize] = Some(block_set.color.clone());
                }
                self.block_set = None;

                self.erase_block_line();
            }
        }
    }

    fn rotate_block_set(&mut self, clockwise: bool) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut cloned = block_set.clone();

            for (x, y) in cloned.content.iter_mut() {
                let (origin_x, origin_y) = cloned.rotation_origin;
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

            if self.is_valid_placement(&cloned) {
                self.block_set = Some(cloned);
            }
        }
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

    fn down_block_set(&mut self) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut cloned = block_set.clone();

            cloned.y -= 1;

            if self.is_valid_placement(&cloned) {
                self.block_set = Some(cloned);
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

    pub fn context(&'_ self) -> GameContext<'_> {
        GameContext {
            blocks: &self.blocks,
            block_set: &self.block_set,
            score: &self.score,
            paused: &self.paused,
        }
    }
}
