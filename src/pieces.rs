
use bevy::prelude::*;
use crate::{constants::*, coord_maker_piece};

#[derive(Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Component, Debug)]
pub struct Piece {
    ptype: PieceType,
    color: PieceColor,
}

pub fn piece_asset_path(piece: &Piece) -> String {
    let mut path = String::new();

    match piece.color {
        PieceColor::White => path.push('w'),
        PieceColor::Black => path.push('b'),
    }
    match piece.ptype {
        PieceType::Pawn => path.push_str("pawn.png"),
        PieceType::Knight => path.push_str("knight.png"),
        PieceType::Bishop => path.push_str("bishop.png"),
        PieceType::Rook => path.push_str("rook.png"),
        PieceType::Queen => path.push_str("queen.png"),
        PieceType::King => path.push_str("king.png")
    }
    path
}

/// Spawns all 32 Chess pieces.
pub fn spawn_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let piece_ranks = vec![0, 1, 6, 7];

    for rank in piece_ranks {
        for file in 0..NUM_SQUARES {
            let (piece_type, piece_color) = match (rank, file) {
                (1, _) => (PieceType::Pawn, PieceColor::White),
                (6, _) => (PieceType::Pawn, PieceColor::Black),
                (_other_rank, _other_file) => {
                    let nested_type = match file {
                        0 | 7 => PieceType::Rook,
                        1 | 6 => PieceType::Knight,
                        2 | 5 => PieceType::Bishop,
                        3 => PieceType::Queen,
                        4 => PieceType::King,
                        _ => PieceType::King, // not possible
                    };    
                    let nested_color = match rank {
                        0 | 1 => PieceColor::White,
                        6 | 7 => PieceColor::Black,
                        _ => PieceColor::White, // not possible
                    };
                    (nested_type, nested_color)
                }
            };

            let piece = Piece {
                ptype:  piece_type,
                color: piece_color,
            };

            let rank_coord = SQUARE_SIZE * (rank as f32);
            let file_coord = SQUARE_SIZE * (file as f32);
            let piece_coords = coord_maker_piece(file_coord, rank_coord);
            spawn_single_piece(&mut commands,
                &asset_server,
                piece,
                piece_coords);

        }
    }

    
}

pub fn spawn_single_piece(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    piece: Piece,
    piece_coords: Vec3
) {

    let asset_path = piece_asset_path(&piece);
    //println!("Spawning: {:?} at {:?}", piece, piece_coords);

    // just testing pawn placement
    commands.spawn(SpriteBundle {
        texture: asset_server.load(asset_path),
        sprite: Sprite {
            color: Color::srgb(5.0, 5.0, 5.0),
            custom_size: Some(Vec2::new(1.0, 1.0)), // in terms of square size??
            ..default()
        },
        transform: Transform {
            translation: piece_coords,
            scale: Vec3::new(SQUARE_SIZE, SQUARE_SIZE, Z + 1.),
            ..default()
        },
        ..default()
    })
    .insert(piece);
}

/// Can't make an official test since it requires args
pub fn test_query_pieces(query: Query<&Piece>) {
    // verify white pieces
    let white_pieces = pieces_by_color(&query, PieceColor::White);
    for piece in white_pieces {
        println!("Found: {:?}", piece);
    }
    println!("~~~~~~~~");
    // verify black pieces 
    let black_pieces = pieces_by_color(&query, PieceColor::Black);
    for piece in black_pieces {
        println!("Found: {:?}", piece);
    }
}

pub fn pieces_by_color<'a>(
    query: &'a Query<&Piece>,
    color: PieceColor
) -> Vec<&'a Piece> {
    query
        .iter()
        .filter(|piece| piece.color == color)
        .collect::<Vec<&'a Piece>>()
}