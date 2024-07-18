

use bevy::prelude::*;
use bevy::color::palettes::basic::{WHITE, BLUE};
use bevy::sprite::MaterialMesh2dBundle;


//////// CONSTANTS ////////
const WINDOW_SIZE: f32 = 800.;
const SQUARE_SIZE: f32 = 100.;
const NUM_SQUARES: i32 = 8;
const Z: f32 = 0.;
const DISPLACEMENT: f32 = 0.5 * (WINDOW_SIZE - SQUARE_SIZE);
///////////////////////////

#[derive(Component)]
struct Square;

fn coord_maker(x: f32, y: f32) -> Vec3 {
    // reduce x and y coords by the size of the board
    Vec3::new(x - DISPLACEMENT, y - DISPLACEMENT, Z)
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn grid_maker(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    let mut coord_indices: Vec<Vec<i32>> = vec![];

    for i in 0..NUM_SQUARES {
        for j in 0..NUM_SQUARES {
            coord_indices.push(vec![i, j]);
        }
    }

    for coord_index in coord_indices {
        let i = coord_index[0] as f32 * SQUARE_SIZE;
        let j = coord_index[1] as f32 * SQUARE_SIZE;

        let color = if (coord_index[0] + coord_index[1]) % 2 == 0 {
            Color::from(BLUE)
        } else {
            Color::from(WHITE)
        };

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform {
                translation: coord_maker(i, j),
                scale: Vec3::new(SQUARE_SIZE, SQUARE_SIZE, Z),
                ..default()
            },
            material: materials.add(color),
            ..default()
        })
        .insert(Square);

    }


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
