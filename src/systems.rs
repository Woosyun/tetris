use bevy::prelude::*;
use crate::components::*;
use rand::{
    prelude::*,
    distr::StandardUniform,
};
use std::collections::HashSet;

pub struct TetrisPlugin;
impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Grid::new())
            .insert_resource(TetrisTimer::new())
            .add_message::<RedrawGridMessage>()
            .add_message::<CheckLinesMessage>()
            .add_systems(Startup, draw_grid)
            .add_systems(Update, (tick, handle_keyboard_input, check_lines, redraw_grid));

    }
}

fn tick(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<TetrisTimer>,
    mut block_query: Query<(Entity, &mut Block), With<Active>>,
    mut grid: ResMut<Grid>,
    mut check_lines_message_writer: MessageWriter<CheckLinesMessage>,
    mut redraw_grid_event: MessageWriter<RedrawGridMessage>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }

    // if activated block exists, pull down. if it reached bottommost, deactivate
    // else, create new one
    if let Ok((entity, mut block)) = block_query.single_mut() {
        let is_crash = block.next_occupancy((-1, 0)).iter()
            .any(|&(row, col)| grid.is_occupied(row, col));

        if is_crash {
            for i in 0..4 {
                for j in 0..4 {
                    if block.shape[i][j] == false { continue }
                    
                    let row = block.position.0 + i as isize;
                    let col = block.position.1 + j as isize;
                    //todo: if row >= GRID_ROWS then send GameOverMessage
                    if row as usize >= GRID_ROWS { 
                        println!("game over!!");
                        continue
                    }
                    grid.cells[row as usize][col as usize] = CellState::Filled(block.get_color());
                }
            }
            commands.entity(entity).despawn();
            check_lines_message_writer.write(CheckLinesMessage);
        } else {
            block.position.0 -= 1;
            redraw_grid_event.write(RedrawGridMessage);
        }
    } else {
        commands.spawn((
            Block::new(rand::rng().sample(StandardUniform)),
            Active,
        ));

        println!("create new block");
    }
}
fn handle_keyboard_input(
    //commands: Commands,
    mut block_query: Query<(Entity, &mut Block), With<Active>>,
    grid: ResMut<Grid>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut redraw_grid_event: MessageWriter<RedrawGridMessage>,
) {
    let (_entity, mut block) = match block_query.iter_mut().next() {
        Some(eb) => eb,
        None => return
    };

    for &k in keyboard_input.get_just_pressed() {
        match k {
            KeyCode::Space => {
                let delta = (-1, 0);
                while block
                    .next_occupancy(delta)
                    .iter()
                    .all(|&(row, col)| !grid.is_occupied(row, col))
                {
                    block.move_delta(delta);
                }
            },
            KeyCode::ArrowUp => {
                //todo: handle when rotating goes out coordinate
                block.rotate();
            },
            KeyCode::ArrowLeft => {
                let conflict = block.next_occupancy((0, -1))
                    .iter()
                    .any(|&(row, col)| grid.is_occupied(row, col));
                if !conflict {
                    block.move_delta((0, -1));
                }
            },
            KeyCode::ArrowRight => {
                let conflict = block.next_occupancy((0, 1))
                    .iter()
                    .any(|&(row, col)| grid.is_occupied(row, col));
                if !conflict {
                    block.move_delta((0, 1));
                }
            },
            KeyCode::ArrowDown => {
                let conflict = block.next_occupancy((-1, 0))
                    .iter()
                    .any(|&(row, col)| grid.is_occupied(row, col));
                if !conflict {
                    block.move_delta((-1, 0));
                }
            },
            _ => continue
        }
    }

    redraw_grid_event.write(RedrawGridMessage);
}

fn check_lines(
    mut grid: ResMut<Grid>,
    mut check_lines_event: MessageReader<CheckLinesMessage>,
    mut redraw_grid_event: MessageWriter<RedrawGridMessage>,
) {
    if check_lines_event.is_empty() {return;}
    check_lines_event.clear();

    let filled_rows = (0..GRID_ROWS)
        .filter(|&row| {
            (0..GRID_COLS).all(|col| {
                grid.cells.get(row).unwrap().get(col).unwrap() != &CellState::Empty
            })
        })
        .collect::<Vec<_>>();
    
    let _ = match filled_rows.get(0) {
        Some(&idx) => {
            grid.cells.drain(idx..idx+filled_rows.len());
        },
        None => ()
    };
    grid.cells.extend(vec![vec![CellState::Empty; GRID_COLS]; filled_rows.len()]);

    redraw_grid_event.write(RedrawGridMessage);
}

fn redraw_grid(
    mut commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    grid: Res<Grid>,
    block_query: Query<&Block, With<Active>>,
    mut cell_query: Query<Entity, With<Cell>>,
    mut redraw_grid_message_reader: MessageReader<RedrawGridMessage>
) {
    if redraw_grid_message_reader.is_empty() { return; }
    redraw_grid_message_reader.clear();

    for entity in cell_query.iter_mut() {
        commands.entity(entity)
            .despawn();
    }
    draw_grid(commands, materials, meshes, grid, block_query);
}

fn draw_grid(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    grid: Res<Grid>,
    block_query: Query<&Block, With<Active>>
) {
    let mut active_cells = HashSet::new();
    let block_color = match block_query.single() {
        Ok(block) => {
            for i in 0..4 {
                for j in 0..4 {
                    if block.shape[i][j] == true {
                        active_cells.insert((i as isize + block.position.0, j as isize + block.position.1));
                    }
                }
            }
            block.get_color()
        },
        _ => Default::default()
    };

    for row in 0..GRID_ROWS + GRID_HIDDEN_ROWS {
        for col in 0..GRID_COLS {
            if row >= GRID_ROWS {
                continue;
            }

            let color = match grid.cells.get(row).unwrap().get(col).unwrap() {
                CellState::Empty => {
                    if active_cells.get(&(row as isize, col as isize)).is_some() {
                        block_color.clone()
                    } else {
                        Color::srgb(0.12, 00.12, 0.10)
                    }
                },
                CellState::Filled(color) => color.clone()
            };

            let cell_x = grid.x + col as f32 * GRID_CELL_SIZE;
            let cell_y = grid.y + row as f32 * GRID_CELL_SIZE;

            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(cell_x, cell_y, 0.)
                    .with_scale(Vec3::new(GRID_CELL_SIZE, GRID_CELL_SIZE, 1.0)),
                Cell,
            ));
        }
    }
}