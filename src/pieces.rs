
use bevy::prelude::*;
use crate::{constants::*, coord_maker_piece, get_square_name};

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
pub struct HighlightedPiece;

#[derive(Component, Debug)]
pub struct Piece {
    pub ptype: PieceType,
    pub color: PieceColor,
    pub square_name: String,
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
            let (ptype, color) = match (rank, file) {
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

            let rank_coord = SQUARE_SIZE * (rank as f32);
            let file_coord = SQUARE_SIZE * (file as f32);
            let piece_coords = coord_maker_piece(file_coord, rank_coord);
            let square_name = get_square_name(rank, file)
                .unwrap();

            
            let piece = Piece {
                ptype,
                color,
                square_name,
            };

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
pub fn _test_query_pieces(query: Query<(Entity, &Piece, &Transform), Without<HighlightedPiece>>) {
    // verify white pieces
    let white_pieces = pieces_by_color(&query, PieceColor::White);
    for (_entity, piece, transform) in white_pieces {
        println!("Found: {:?} at {:?}", piece, transform.translation);
    }
    println!("~~~~~~~~");
    // verify black pieces 
    let black_pieces = pieces_by_color(&query, PieceColor::Black);
    for (_entity, piece, transform) in black_pieces {
        println!("Found: {:?} at {:?}", piece, transform.translation);
    }
}

pub fn pieces_by_color<'a>(
    query: &'a Query<(Entity, &Piece, &Transform), Without<HighlightedPiece>>,
    color: PieceColor
) -> Vec<(Entity, &'a Piece, &'a Transform)> {
    query
        .iter()
        .filter(|(_entity, piece, _transform)| piece.color == color)
        .collect::<Vec<(Entity, &'a Piece, &Transform)>>()
}

pub fn _test_piece_movement(mut query: Query<(&Piece, &mut Transform)>) {
    let e2_pos = Vec3::new(50.0, -250.0, 1.0);
    for (_piece, mut transform) in query.iter_mut() {
        if transform.translation == e2_pos {
            transform.translation += Vec3::new(0.0, 200., 0.);
        }
    }
}

/// Generate available moves for a selected piece
pub fn generate_moves(
    selected_piece: &Piece,
    pieces_query: Query<&Piece, Without<HighlightedPiece>>
) -> Vec<String> {
    let other_pieces: Vec<&Piece> = pieces_query.iter().collect();

    match selected_piece.ptype {
        PieceType::Pawn => pawn_moves(selected_piece, other_pieces),
        PieceType::Knight => knight_moves(selected_piece, other_pieces),
        PieceType::Bishop => bishop_moves(selected_piece, other_pieces),
        PieceType::Rook => rook_moves(selected_piece, other_pieces),
        PieceType::Queen => queen_moves(selected_piece, other_pieces),
        PieceType::King => king_moves(selected_piece, other_pieces),
    }
}

/// Generates possible moves for a selected Pawn
pub fn pawn_moves(piece: &Piece, other_pieces: Vec<&Piece>) -> Vec<String> {
    let mut moves = Vec::new();

    moves
}

/// Generates possible moves for a selected Knight
pub fn knight_moves(piece: &Piece, other_pieces: Vec<&Piece>) -> Vec<String> {
    let mut moves = Vec::new();

    moves
}

/// Generates possible moves for a selected Bishop
pub fn bishop_moves(piece: &Piece, other_pieces: Vec<&Piece>) -> Vec<String> {
    let mut moves = Vec::new();

    moves
}

/// Generates possible moves for a selected Rook
pub fn rook_moves(piece: &Piece, other_pieces: Vec<&Piece>) -> Vec<String> {
    let mut moves = Vec::new();

    moves
}

/// Generates possible moves for a selected Queen
pub fn queen_moves(piece: &Piece, other_pieces: Vec<&Piece>) -> Vec<String> {
    let mut moves = Vec::new();

    moves
}

/// Generates possible moves for a selected King
pub fn king_moves(piece: &Piece, other_pieces: Vec<&Piece>) -> Vec<String> {
    let mut moves = Vec::new();

    moves
}