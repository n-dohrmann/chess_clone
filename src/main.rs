

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::constants::*;
use crate::board::*;

mod constants;
mod board;


fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (WINDOW_SIZE, WINDOW_SIZE).into(),
            title: String::from("Chess Clone"),
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, (camera_setup, grid_maker))
    .run();
}
