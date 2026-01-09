use bevy::prelude::*;
use crate::grid::{Grid, GRID_ROWS, GRID_COLS};
use rand::{
    distr::{Distribution, StandardUniform},
    Rng
};

pub struct BlocksPlugin;
impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        todo!();
    }
}

#[derive(Component)]
pub struct Active;

// represent block with 4x4 boolean values
#[derive(Component)]
pub struct Block {
    pub shape: [[bool; 4]; 4],
    pub position: (isize, isize), // (row, column), row can go minus
    pub letter: BlockLetter,
}
impl Block {
    fn new(block_letter: BlockLetter) -> Self {
        let shape = match block_letter {
            BlockLetter::I => [
                [true, false, false, false],
                [true, false, false, false],
                [true, false, false, false],
                [true, false, false, false]
            ],
            BlockLetter::L=> [
                [false, false, false, false],
                [true, false, false, false],
                [true, false, false, false],
                [true, true, false, false],
            ],
            BlockLetter::J=> [
                [false, false, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [true, true, false, false],
            ],
            BlockLetter::O=> [
                [false, false, false, false],
                [false, false, false, false],
                [true, true, false, false],
                [true, true, false, false],
            ],
            BlockLetter::T=> [
                [false, false, false, false],
                [false, false, false, false],
                [true, true, true, false],
                [false, true, false, false],
            ],
            BlockLetter::S=> [
                [false, false, false, false],
                [true, false, false, false],
                [true, true, false, false],
                [false, true, false, false],
            ],
            BlockLetter::Z=> [
                [false, false, false, false],
                [false, true, false, false],
                [true, true, false, false],
                [true, false, false, false],
            ],
        };
        let position = (
            0, // x (= col)
            GRID_ROWS as isize - 5 // y (= row)
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
        for i in 0..4 {
            for j in (i+1)..4 {
                let tmp = self.shape[i][j];
                self.shape[i][j] = self.shape[j][i];
                self.shape[j][i] = tmp;
            }
        }

        self.shape.reverse();
    }

    pub fn clear_row(&mut self, row: usize) {
        assert!(row < 4);

        for col in 0..4 {
            self.shape[row][col] = false;
        }

        for i in row..3 {
            self.shape[row] = self.shape[row+1];
        }
        self.shape[3] = [false, false, false, false];
    }

    pub fn move_down(&mut self) {
        self.position.0 -= 1;
    }
    pub fn move_left(&mut self) {
        self.position.1 -= 1;
    }
    pub fn move_right(&mut self) {
        self.position.1 += 1;
    }

    pub fn empty(&self) -> bool {
        !self.shape.iter()
            .find(|&row| {
                row.iter()
                    .find(|&&el| el)
                    .is_some()
            })
            .is_some()
    }
}

#[derive(Clone, Debug)]
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
        match rng.random_range(0..7) {
            0 => BlockLetter::I,
            1 => BlockLetter::L,
            2 => BlockLetter::J,
            3 => BlockLetter::O,
            4 => BlockLetter::T,
            5 => BlockLetter::S,
            6 => BlockLetter::Z,
        }
    }
}

fn spawn_block(
    mut commands: Commands,
) {
    let new_block_letter: BlockLetter = rand::random();
    let new_block = Block::from(new_block_letter);
    commands.spawn((new_block, Active));
}
// todo: deactive_block

fn move_block(
    mut commands: Commands,
    mut blocks: Query<(Entity, &mut Block), With<Active>>,
    grid: Res<Grid>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (entity, mut block) = match blocks.get_mut() {
        Ok(re) => re,
        _ => return;
    };

    for &k in keyboard_input.get_just_pressed() {
        match k {
            //todo: check collision between blockb - block, block - wall
            KeyCode::ArrowLeft => {
                if hit_left(grid, block) {
                    continue;
                }
                block.move_left();
            },
            KeyCode::ArrowRight => {
                if hit_right(grid, block) {
                    continue;
                }
                block.move_right();
            },
            KeyCode::ArrowUp => {
                todo!("???");
            },
            KeyCode::ArrowDown => {
                if hit_bottom(grid, block) {
                    continue;
                }
                block.move_down();
            },
            KeyCode::Space => {
                while !hit_bottom(grid, block) {
                    block.move_down();
                }
            },
            _ => (),
        }
    }

    //redraw?
}

fn leftmost(grid: &Grid, block: &Block) -> bool {
    let (x, y) = block.position;
    for i in 0..4 {
        for j in 0..4 {
            let row = x + i;
            let col = y + j;
            if row < 0 || col < 0 {
                continue;
            }

            if col == 0 || !grid.cell(row, col - 1).is_empty() {
                return true;
            }
        }
    }
    return false;
}
fn rightmost(grid: &Grid, block: &Block) -> bool {}
fn bottommost(grid: &Grid, block: &Block) -> bool {}
