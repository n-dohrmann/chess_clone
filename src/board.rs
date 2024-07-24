
use bevy::prelude::*;
use bevy::color::palettes::basic::{SILVER, BLUE};
use bevy::sprite::MaterialMesh2dBundle;
use crate::constants::*;

#[derive(Component, Debug, PartialEq)]
pub struct Square {
    pub name: String,
    pub color: Color,
}

#[derive(Component, Debug)]
pub struct HighlightedSquare;

#[derive(Resource, Debug)]
pub struct Bins(pub Vec<f32>);

impl Default for Bins {
   fn default() -> Self {
       let mut bins: Vec<f32> = Vec::new();
       for i in 0..NUM_SQUARES {
            bins.push(-350. + i as f32 * 100.);
       }
       Bins(bins)
   }
}

pub fn coord_maker(x: f32, y: f32) -> Vec3 {
    Vec3::new(x - DISPLACEMENT, y - DISPLACEMENT, Z)
}

pub fn coord_maker_piece(x: f32, y: f32) -> Vec3 {
    // Add one to z layer for the pieces
    Vec3::new(x - DISPLACEMENT, y - DISPLACEMENT, Z + 1.)
}

pub fn get_square_name(mut rank_num: i32, mut file_num: i32) -> Result<String, &'static str> {
    //rank_num = 7  - (rank_num % 8);
    //file_num = 7  - (file_num % 8);
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
        7 => "8",
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
        let square_name = get_square_name(j, i).unwrap();

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
        .insert(Square{
            name: square_name,
            color: color,
        });

    }
}

pub fn get_rkfl_from_coords(coords: Vec3) -> (i32, i32) {
    let rank = ((coords.y + 350.) / 100.) as i32;
    let file = ((coords.x + 350.) / 100.) as i32;
    (rank, file)
}

/// Generate a coordinate (`Vec3`) corresponding to a square
/// name. Ex. `"A1" -> Vec3::new(-350., -350., 1.0)`
pub fn square_name_to_coords(square_name: &String) -> Vec3 {
    let mut coords = Vec3::new(0., 0., 1.0);

    let file: &str = &square_name[0..1];
    let rank: &str = &square_name[1..2];

    match file {
        "A" => coords.x = -350.,
        "B" => coords.x = -250.,
        "C" => coords.x = -150.,
        "D" => coords.x = -50.,
        "E" => coords.x = 50.,
        "F" => coords.x = 150.,
        "G" => coords.x = 250.,
        "H" => coords.x = 350.,
        _ => coords.x = 350., // not possible
    }
    match rank {
        "1" => coords.y = -350.,
        "2" => coords.y = -250.,
        "3" => coords.y = -150.,
        "4" => coords.y = -50.,
        "5" => coords.y = 50.,
        "6" => coords.y = 150.,
        "7" => coords.y = 250.,
        "8" => coords.y = 350.,
        _ => coords.y = 350., // not possible
    }
    coords
}