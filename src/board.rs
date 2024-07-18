
use bevy::prelude::*;
use bevy::color::palettes::basic::{SILVER, BLUE};
use bevy::sprite::MaterialMesh2dBundle;
use crate::constants::*;

#[derive(Component)]
pub struct Square {
    pub name: String,
}


pub fn coord_maker(x: f32, y: f32) -> Vec3 {
    Vec3::new(x - DISPLACEMENT, y - DISPLACEMENT, Z)
}

pub fn coord_maker_piece(x: f32, y: f32) -> Vec3 {
    // Add one to z layer for the pieces
    Vec3::new(x - DISPLACEMENT, y - DISPLACEMENT, Z + 1.)
}

pub fn get_square_name(mut rank_num: i32, mut file_num: i32) -> Result<String, &'static str> {
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

pub fn grid_maker(
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
            Color::from(SILVER)
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
        .insert(Square{ name: square_name });

    }
}