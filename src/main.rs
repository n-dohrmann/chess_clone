
use bevy::prelude::*;

use crate::constants::*;
use crate::board::*;
use crate::pieces::*;
use crate::game_logic::*;
use crate::mouse_input::*;

pub mod constants;
pub mod board;
pub mod pieces;
pub mod game_logic;
pub mod mouse_input;

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
    .insert_resource(GameState::White)
    .insert_resource(Bins::default())
    .add_systems(Startup, (
        camera_setup,
        grid_maker,
        spawn_pieces,
    ))
    .add_systems(Update, _test_highlight_when_clicked)
    .run();
}
