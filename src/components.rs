use bevy::prelude::*;
use rand::{
    distr::{Distribution, StandardUniform},
    Rng
};

pub const GRID_COLS: usize = 10;
pub const GRID_ROWS: usize = 20;
pub const GRID_HIDDEN_ROWS: usize = 6;
pub const GRID_CELL_SIZE: f32 = 20.0;

#[derive(Message)]
pub struct RedrawGridMessage;
#[derive(Message)]
pub struct DeactivateBlockMessage;
#[derive(Message)]
pub struct CheckLinesMessage;

#[derive(Resource)]
pub struct Grid {
    pub cells: Vec<Vec<CellState>>,
    pub x: f32,
    pub y: f32,
}
impl Grid {
    pub fn new() -> Self {
        Self {
            //cells: vec![CellState::Empty; GRID_COLS * (GRID_ROWS + GRID_HIDDEN_ROWS)],
            cells: vec![vec![CellState::Empty; GRID_COLS]; GRID_ROWS],
            x: -(GRID_COLS as f32 * GRID_CELL_SIZE) / 2.0,
            y: -(GRID_ROWS as f32 * GRID_CELL_SIZE) / 2.0,
        }
    }
    pub fn is_occupied(&self, row: isize, col: isize) -> bool {
        row < 0
        || col < 0
        || col >= GRID_COLS as isize
        || (row < GRID_ROWS as isize && self.cells.get(row as usize).unwrap().get(col as usize).unwrap() != &CellState::Empty)
    }
}
#[derive(Component)]
pub struct Cell;
#[derive(Clone, PartialEq)]
pub enum CellState {
    Empty,
    Filled(Color)
}

#[derive(Component)]
pub struct Active;

#[derive(Component, Clone)]
pub struct Block {
    pub shape: [[bool; 4]; 4],
    pub letter: BlockLetter,
    pub position: (isize, isize)
}
impl Block {
    pub fn new(block_letter: BlockLetter) -> Self {
        let shape = match block_letter {
            BlockLetter::I => [
                [true, false, false, false],
                [true, false, false, false],
                [true, false, false, false],
                [true, false, false, false]
            ],
            BlockLetter::L=> [
                [true, true, false, false],
                [true, false, false, false],
                [true, false, false, false],
                [false, false, false, false],
            ],
            BlockLetter::J=> [
                [true, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            BlockLetter::O=> [
                [true, true, false, false],
                [true, true, false, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
            BlockLetter::T=> [
                [true, true, true, false],
                [false, true, false, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
            BlockLetter::S=> [
                [false, true, false, false],
                [true, true, false, false],
                [true, false, false, false],
                [false, false, false, false],
            ],
            BlockLetter::Z=> [
                [true, false, false, false],
                [true, true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
        };
        let position = (
            GRID_ROWS as isize, // row
            GRID_COLS as isize / 2 - 2, // col
        );
        Self {
            shape, 
            position, 
            letter: block_letter
        }
    }

    pub fn get_color(&self) -> Color {
        match self.letter.clone() {
            BlockLetter::I => Color::srgb(0.0, 0.95, 1.0),
            BlockLetter::L => Color::srgb(0.1, 0.3, 0.8),
            BlockLetter::J => Color::srgb(1.0, 0.6, 0.2),
            BlockLetter::O => Color::srgb(1.0, 0.85, 0.2),
            BlockLetter::T => Color::srgb(0.0, 0.8, 0.4),
            BlockLetter::S => Color::srgb(0.85, 0.1, 0.3),
            BlockLetter::Z => Color::srgb(0.7, 0.3, 0.8),
        }
    }

    pub fn rotate(&mut self) {
        let mut re = [[false; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                re[i][j] = self.shape[j][3-i];
            }
        }

        let mut leftmost_idx = 3;
        let mut bottommost_idx = 3;
        for i in 0..4 {
            for j in 0..4 {
                if !re[i][j] { continue }
                if leftmost_idx > j {
                    leftmost_idx = j;
                }
                if bottommost_idx > i {
                    bottommost_idx = i;
                }
            }
        }
        for i in 0..4 {
            for j in 0..4 {
                if i < 4 - bottommost_idx && j < 4 - leftmost_idx {
                    re[i][j] = re[i+bottommost_idx][j+leftmost_idx];
                } else {
                    re[i][j] = false;
                }
            }
        }

        self.shape = re;
    }

    pub fn move_delta(&mut self, delta: (isize, isize)) {
        match delta {
            (0, 1) => self.position.1 += 1,
            (-1, 0) => self.position.0 -= 1,
            (0, -1) => self.position.1 -= 1,
            _ => panic!("illegal movement: {:?}", delta)
        }
    }

    pub fn next_occupancy(&self, delta: (isize, isize)) -> Vec<(isize, isize)> {
        let mut re = vec![];
        let (row, col) = self.position;

        for i in 0..4 {
            for j in 0..4 {
                if self.shape[i][j] == false { continue }
                re.push((row + i as isize + delta.0, col + j as isize + delta.1));
            }
        }

        re
    }
}

#[derive(Clone)]
pub enum BlockLetter {
    I,
    L,
    J,
    O,
    T,
    S,
    Z,
}
impl Distribution<BlockLetter> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockLetter {
        match rng.random_range(0..=6) {
            0 => BlockLetter::I,
            1 => BlockLetter::L,
            2 => BlockLetter::J,
            3 => BlockLetter::O,
            4 => BlockLetter::T,
            5 => BlockLetter::S,
            _ => BlockLetter::Z
        }
    }
}

#[derive(Resource)]
pub struct TetrisTimer(pub Timer);
impl TetrisTimer {
    pub fn new() -> Self {
        Self(Timer::new(std::time::Duration::new(1, 0), TimerMode::Repeating))
    }
}