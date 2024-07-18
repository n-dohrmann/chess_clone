

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
struct Square(String);

fn coord_maker(x: f32, y: f32) -> Vec3 {
    // reduce x and y coords by the size of the board
    Vec3::new(x - DISPLACEMENT, y - DISPLACEMENT, Z)
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn get_square_name(mut rank_num: i32, mut file_num: i32) -> Result<String, &'static str> {
    rank_num = rank_num % 8;
    file_num = file_num % 8;
    let rank = match rank_num {
        0 => "1",
        1 => "2",
        2 => "3",
        3 => "4",
        4 => "5",
        5 => "6",
        6 => "7",
        7 => "7",
        _ => return Err("invalid rank!") // not possible...
    };

    let mut file = match file_num {
        0 => String::from("A"),
        1 => String::from("B"),
        2 => String::from("C"),
        3 => String::from("D"),
        4 => String::from("E"),
        5 => String::from("F"),
        6 => String::from("G"),
        7 => String::from("H"),
        _ => return Err("invalid rank!")
    };

    file.push_str(rank);
    Ok(String::from(file))
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
        let i = coord_index[0];
        let j = coord_index[1];
        let i_scaled = i as f32 * SQUARE_SIZE;
        let j_scaled = j as f32 * SQUARE_SIZE;

        // this should never give an err
        let square_name = get_square_name(i, j).unwrap();

        let color = if (coord_index[0] + coord_index[1]) % 2 == 0 {
            Color::from(BLUE)
        } else {
            Color::from(WHITE)
        };

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform {
                translation: coord_maker(i_scaled, j_scaled),
                scale: Vec3::new(SQUARE_SIZE, SQUARE_SIZE, Z),
                ..default()
            },
            material: materials.add(color),
            ..default()
        })
        .insert(Square(square_name));

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
