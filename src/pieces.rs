
use bevy::prelude::*;
use crate::{constants::*, coord_maker_piece};
use crate::board::{Square, coord_maker};

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub enum PieceColor {
    White,
    Black,
}

#[derive(Component)]
pub struct Piece {
    ptype: PieceType,
    color: PieceColor,
}

pub fn spawn_piece(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //piece: Piece,
    //square: Square
) {
    //let square_name = square.name;

    //let texture: Handle<Image> = match piece.ptype {
        //PieceType::Pawn => asset_server.load("assets/pbawn.png"),
        //_ => asset_server.load("assets/bpawn.png"),
    //};

    // just testing pawn placement
    commands.spawn(SpriteBundle {
        texture: asset_server.load("bpawn.png"),
        sprite: Sprite {
            color: Color::srgb(5.0, 5.0, 5.0),
            custom_size: Some(Vec2::new(1.0, 1.0)), // in terms of square size??
            ..default()
        },
        transform: Transform {
            translation: coord_maker_piece(0., 600.),
            scale: Vec3::new(SQUARE_SIZE, SQUARE_SIZE, Z + 1.),
            ..default()
        },
        ..default()
    });
    //.insert(Piece);

    commands.spawn(SpriteBundle {
        texture: asset_server.load("wpawn.png"),
        sprite: Sprite {
            color: Color::srgb(5.0, 5.0, 5.0),
            custom_size: Some(Vec2::new(1.0, 1.0)), // in terms of square size??
            ..default()
        },
        transform: Transform {
            translation: coord_maker_piece(0., 100.),
            scale: Vec3::new(SQUARE_SIZE, SQUARE_SIZE, Z + 1.),
            ..default()
        },
        ..default()
    });


}