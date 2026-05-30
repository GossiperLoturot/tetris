use std::collections::HashSet;
use std::time::{Duration, Instant};

use rand::prelude::*;

use crate::{consts, game};

#[derive(Clone)]
pub struct Mino {
    pub x: i32,
    pub y: i32,
    pub blocks: Vec<(i32, i32)>,
    pub template: consts::MinoTemplate,
}

pub struct GameContext<'a> {
    pub active_mino: &'a Option<Mino>,
    pub blocks: &'a Vec<Vec<Option<consts::BlockColor>>>,
    pub score: &'a i32,
    pub paused: &'a bool,
}

pub struct GameSystem {
    rng: ThreadRng,
    pressed: HashSet<winit::keyboard::KeyCode>,
    last_update: Option<Instant>,
    remaining_time: Duration,

    active_mino: Option<Mino>,
    blocks: Vec<Vec<Option<consts::BlockColor>>>,

    paused: bool,
    score: i32,
}

impl GameSystem {
    pub fn new() -> Self {
        Self {
            rng: rand::rng(),
            pressed: HashSet::new(),
            last_update: None,
            remaining_time: Duration::ZERO,

            active_mino: None,
            blocks: vec![
                vec![None; consts::MAX_BLOCK_WIDTH as usize];
                consts::MAX_BLOCK_HEIGHT as usize
            ],

            paused: false,
            score: 0,
        }
    }

    pub fn input(&mut self, input: &winit::event::KeyEvent, flow: &mut game::GameSystemFlow) {
        use winit::event::ElementState;
        use winit::keyboard::KeyCode;

        if let winit::keyboard::PhysicalKey::Code(code) = input.physical_key {
            match input.state {
                ElementState::Pressed if !self.pressed.contains(&code) => {
                    match code {
                        KeyCode::KeyP => {
                            self.paused = !self.paused;

                            if !self.paused {
                                self.last_update = Some(Instant::now());
                            }
                        }
                        KeyCode::Space if !self.paused => {
                            self.check_and_hard_drop_mino(flow);
                        }
                        KeyCode::ArrowUp | KeyCode::KeyX if !self.paused => {
                            self.check_and_rotate_mino(true);
                        }
                        KeyCode::KeyZ if !self.paused => {
                            self.check_and_rotate_mino(false);
                        }
                        KeyCode::ArrowDown if !self.paused => {
                            self.check_and_move_mino(0, -1);
                        }
                        KeyCode::ArrowRight if !self.paused => {
                            self.check_and_move_mino(1, 0);
                        }
                        KeyCode::ArrowLeft if !self.paused => {
                            self.check_and_move_mino(-1, 0);
                        }
                        _ => {}
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

    pub fn update(&mut self, flow: &mut game::GameSystemFlow) {
        let delta_time = self.last_update.map(|last_update| last_update.elapsed());
        self.last_update = Some(std::time::Instant::now());

        if self.paused {
            return;
        }

        if let Some(delta_time) = delta_time {
            self.remaining_time += delta_time;
        }

        if consts::UPDATE_INTERVAL < self.remaining_time {
            self.check_and_place_mino(flow);

            self.check_and_move_mino(0, -1);

            self.check_and_spawn_mino(flow);

            self.remaining_time = self.remaining_time.saturating_sub(consts::UPDATE_INTERVAL);
        }
    }

    fn is_valid_mino(&self, mino: &Mino) -> bool {
        mino.blocks.iter().all(|(x, y)| {
            let x = mino.x + x;
            let y = mino.y + y;
            (0..consts::MAX_BLOCK_WIDTH).contains(&x)
                && (0..consts::MAX_BLOCK_HEIGHT).contains(&y)
                && self.blocks[y as usize][x as usize].is_none()
        })
    }

    fn check_and_spawn_mino(&mut self, flow: &mut game::GameSystemFlow) {
        if self.active_mino.is_none() {
            let mino_template = consts::MINO_TEMPLATES.choose(&mut self.rng).unwrap();

            let active_mino = Mino {
                x: consts::SPAWN_BLOCK_X,
                y: consts::SPAWN_BLOCK_Y,
                blocks: mino_template.blocks.to_vec(),
                template: mino_template.clone(),
            };

            if !self.is_valid_mino(&active_mino) {
                let state = game::GameSystem::End(game::end::GameSystem::new(
                    self.blocks.clone(),
                    self.score,
                ));
                *flow = game::GameSystemFlow::To(state);
            }

            self.active_mino = Some(active_mino);
        }
    }

    fn check_and_place_mino(&mut self, flow: &mut game::GameSystemFlow) {
        if let Some(active_mino) = self.active_mino.as_ref() {
            let mut next_mino = active_mino.clone();

            next_mino.y -= 1;

            if self.is_valid_mino(active_mino) && !self.is_valid_mino(&next_mino) {
                for (x, y) in active_mino.blocks.iter() {
                    let x = active_mino.x + *x;
                    let y = active_mino.y + *y;
                    self.blocks[y as usize][x as usize] = Some(active_mino.template.color.clone());
                }
                self.active_mino = None;

                self.check_and_erase_blocks();

                let is_over_stack_height = self
                    .blocks
                    .iter()
                    .skip(consts::MAX_STACK_HEIGHT as usize)
                    .any(|line| line.iter().any(|block| block.is_some()));
                if is_over_stack_height {
                    let state = game::GameSystem::End(game::end::GameSystem::new(
                        self.blocks.clone(),
                        self.score,
                    ));
                    *flow = game::GameSystemFlow::To(state);
                }

                self.remaining_time = Duration::ZERO;
            }
        }
    }

    fn check_and_move_mino(&mut self, delta_x: i32, delta_y: i32) {
        if let Some(active_mino) = self.active_mino.as_ref() {
            let mut next_mino = active_mino.clone();

            next_mino.x += delta_x;
            next_mino.y += delta_y;

            if self.is_valid_mino(&next_mino) {
                self.active_mino = Some(next_mino);
            }
        }
    }

    fn check_and_rotate_mino(&mut self, clockwise: bool) {
        if let Some(active_mino) = self.active_mino.as_ref() {
            let mut next_mino = active_mino.clone();

            for (x, y) in next_mino.blocks.iter_mut() {
                let (origin_x, origin_y) = active_mino.template.rotation_origin;

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

            if self.is_valid_mino(&next_mino) {
                self.active_mino = Some(next_mino);
            }
        }
    }

    fn check_and_hard_drop_mino(&mut self, flow: &mut game::GameSystemFlow) {
        if let Some(active_mino) = self.active_mino.as_ref() {
            let mut dropped_mino = active_mino.clone();

            loop {
                let mut next_mino = dropped_mino.clone();

                next_mino.y -= 1;

                if self.is_valid_mino(&next_mino) {
                    dropped_mino = next_mino;
                } else {
                    break;
                }
            }
            self.active_mino = Some(dropped_mino);

            self.check_and_place_mino(flow);
        }
    }

    fn check_and_erase_blocks(&mut self) {
        let mut row_nums = vec![];

        for (row, line) in self.blocks.iter_mut().enumerate() {
            let is_filled_line = line.iter().all(|block| block.is_some());
            if is_filled_line {
                line.iter_mut().for_each(|block| *block = None);
                row_nums.push(row);

                self.score += 1;
            }
        }

        for row in 0..consts::MAX_BLOCK_HEIGHT as usize {
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
            active_mino: &self.active_mino,
            score: &self.score,
            paused: &self.paused,
        }
    }
}
