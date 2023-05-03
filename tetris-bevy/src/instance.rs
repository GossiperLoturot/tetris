use bevy::prelude::*;

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
pub struct BlockSet {
    pub x: i32,
    pub y: i32,
    pub color: BlockColor,
    pub content: Vec<(i32, i32)>,
}

#[rustfmt::skip]
const BLOCK_SET_TABLE: &[(&[(i32, i32)], BlockColor)] = &[
    (&[(-1,  0), ( 0,  0), (1,  0), (2,  0)], BlockColor::Cyan),
    (&[( 0,  0), ( 1,  0), (1,  1), (0,  1)], BlockColor::Yellow),
    (&[(-1, -1), ( 0, -1), (0,  0), (1,  0)], BlockColor::Green),
    (&[(-1,  0), ( 0,  0), (0, -1), (1, -1)], BlockColor::Red),
    (&[(-1,  0), (-1, -1), (0, -1), (1, -1)], BlockColor::Blue),
    (&[(-1, -1), ( 0, -1), (1, -1), (1,  0)], BlockColor::Orange),
    (&[(-1, -1), ( 0, -1), (1, -1), (0,  0)], BlockColor::Purple),
];

const BLOCK_WIDTH: u32 = 10;
const BLOCK_HEIGHT: u32 = 25;
const BLOCK_SPAWN_X: i32 = 5;
const BLOCK_SPAWN_Y: i32 = 20;
const INTERVAL_MSEC: u64 = 400;

#[derive(Resource)]
pub struct Instance {
    pub block_width: u32,
    pub block_height: u32,
    pub block_spawn_x: i32,
    pub block_spawn_y: i32,
    pub block_set: Option<BlockSet>,
    pub blocks: Vec<Vec<Option<BlockColor>>>,
    pub score: u32,
    pub update_interval: std::time::Duration,
    pub last_update: Option<std::time::Instant>,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            block_width: BLOCK_WIDTH,
            block_height: BLOCK_HEIGHT,
            block_spawn_x: BLOCK_SPAWN_X,
            block_spawn_y: BLOCK_SPAWN_Y,
            block_set: None,
            blocks: vec![vec![None; BLOCK_WIDTH as usize]; BLOCK_HEIGHT as usize],
            score: 0,
            update_interval: std::time::Duration::from_millis(INTERVAL_MSEC),
            last_update: None,
        }
    }
}

impl Instance {
    pub fn update(&mut self) {
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
                    // game over
                }
            }

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

    fn spawn_block_set(&mut self) {
        let (content, color) = &BLOCK_SET_TABLE[0]; // TODO: ランダムな選択

        self.block_set = Some(BlockSet {
            x: self.block_spawn_x,
            y: self.block_spawn_y,
            color: color.clone(),
            content: content.to_vec(),
        });
    }

    pub fn place_block_set(&mut self) {
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

    pub fn rotate_block_set(&mut self) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut cloned = block_set.clone();

            for (x, y) in cloned.content.iter_mut() {
                (*x, *y) = (*y, -*x);
            }

            if self.is_valid_placement(&cloned) {
                self.block_set = Some(cloned);
            }
        }
    }

    pub fn down_block_set(&mut self) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut cloned = block_set.clone();

            cloned.y -= 1;

            if self.is_valid_placement(&cloned) {
                self.block_set = Some(cloned);
            }
        }
    }

    pub fn right_block_set(&mut self) {
        if let Some(block_set) = self.block_set.as_ref() {
            let mut cloned = block_set.clone();

            cloned.x += 1;

            if self.is_valid_placement(&cloned) {
                self.block_set = Some(cloned);
            }
        }
    }

    pub fn left_block_set(&mut self) {
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
}
