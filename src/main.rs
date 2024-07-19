

use bevy::prelude::*;

use crate::constants::*;
use crate::board::*;
use crate::pieces::*;

mod constants;
mod board;
mod pieces;


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
    .add_systems(Startup, (
        camera_setup,
        grid_maker,
        spawn_pieces,
        test_query_pieces.after(spawn_pieces),
    ))
    .run();
}
