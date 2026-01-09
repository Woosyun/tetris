use bevy::prelude::*;
pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Grid::new())
            .add_message::<RedrawGridEvent>()
            .add_message::<CheckLinesEvent>()
            .add_systems(Startup, draw_grid)
            .add_systems(Update, (redraw_grid, check_lines));
    }
}

pub const GRID_COLS: usize = 10;
pub const GRID_ROWS: usize = 20;
const GRID_HIDDEN_ROWS: usize = 6;
const GRID_CELL_SIZE: f32 = 40.0;

#[derive(Resource)]
pub struct Grid {
    cells: Vec<CellState>,
    x: f32,
    y: f32,
}
impl Grid {
    fn new() -> Self {
        Self {
            cells: vec![CellState::Empty; GRID_COLS * (GRID_ROWS + GRID_HIDDEN_ROWS)],
            x: -(GRID_ROWS as f32 * GRID_CELL_SIZE) / 2.0,
            y: -(GRID_COLS as f32 * GRID_CELL_SIZE) / 2.0,
        }
    }
}

#[derive(Component)]
struct Cell;

#[derive(Resource, Clone, PartialEq)]
enum CellState {
    Empty,
    Filled(Color),
}

fn draw_grid(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    grid: Res<Grid>,
) {
    //draw cells
    for j in 0..GRID_ROWS + GRID_HIDDEN_ROWS {
        for i in 0..GRID_COLS {
            let index = j * GRID_COLS + i;
            let color = match grid.cells.get(index).unwrap() {
                CellState::Empty => Color::srgb(0.12, 0.12, 0.18),
                CellState::Filled(color) => color.clone()
            };

            let cell_x = grid.x + i as f32 * GRID_CELL_SIZE;
            let cell_y = grid.y + j as f32 * GRID_CELL_SIZE;

            if j < GRID_ROWS {
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_xyz(cell_x, cell_y, -69.0)
                        .with_scale(Vec3::new(GRID_CELL_SIZE, GRID_CELL_SIZE, 1.0)),
                    Cell,
                ));
            }
        }
    }
}

#[derive(Message)]
pub struct RedrawGridEvent;

fn redraw_grid(
    mut commands: Commands,
    mut redraw_grid_events: MessageReader<RedrawGridEvent>,
    materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    grid: Res<Grid>,
    grid_cell_query: Query<(Entity, &Cell)>,
) {
    if !redraw_grid_events.is_empty() {
        redraw_grid_events.clear();

        for (entity, _) in grid_cell_query.iter() {
            commands.entity(entity).despawn();
        }

        draw_grid(commands, materials, meshes, grid);
    }
}

#[derive(Message)]
pub struct CheckLinesEvent;

fn check_lines(
    mut grid: ResMut<Grid>,
    mut check_lines_event: MessageReader<CheckLinesEvent>,
    mut redraw_grid_event: MessageWriter<RedrawGridEvent>,
) {
    if !check_lines_event.is_empty() {
        check_lines_event.clear();
        
        let filled_rows = (0..GRID_ROWS)
            .filter(|&i| {
                (0..GRID_COLS).all(|j| {
                    grid.cells[i * GRID_COLS + j] != CellState::Empty
                })
            })
            .collect::<Vec<_>>();

        // drain filled rows
        for row in &filled_rows {
            grid.cells.drain(row*GRID_COLS..(row+1)*GRID_COLS);
        }
        // fill new empty rows
        grid.cells.extend(vec![CellState::Empty; filled_rows.len() * GRID_COLS]);

        redraw_grid_event.write(RedrawGridEvent);
    }
}
