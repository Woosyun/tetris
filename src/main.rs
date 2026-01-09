use bevy::prelude::*;
use tetris::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "tetris".to_string(),
                    resizable: true,
                    resolution: (360u32, 443u32).into(),
                    ..Default::default()
                }),
                ..Default::default()
            }
        ))
        .add_plugins(grid::GridPlugin)
        //.add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
