
use bevy::{prelude::*, window::PrimaryWindow};
use crate::board::*;
use crate::pieces::*;

use crate::get_name_clicked_square;

#[derive(Resource, Debug)]
pub enum GameState {
    White,
    Black,
}

pub fn toggle_game_state(mut state: ResMut<GameState>) {
    match state.as_mut() {
        GameState::White => *state = GameState::Black,
        GameState::Black => *state = GameState::White,
    }
}

pub fn verify_game_state(state: ResMut<GameState>) {
    println!("The found state is: {:?}", state);
}

pub fn piece_selection_logic(
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    bins: Res<Bins>,
    mut game_state: ResMut<GameState>,
    piece_query: Query<(Entity, &Piece, &Transform), Without<HighlightedPiece>>,
    mut hl_piece_query: Query<(Entity, &Piece, &Transform), With<HighlightedPiece>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(position) = q_windows.single().cursor_position() {
        let name = get_name_clicked_square(position, &bins);

        // get the pieces of the current game state
        let active_pieces = match game_state.as_mut() {
            GameState::White => pieces_by_color(&piece_query, PieceColor::White),
            GameState::Black => pieces_by_color(&piece_query, PieceColor::Black),
        };

        if hl_piece_query.is_empty() { // no currently selected piece
            for (entity, piece, transform) in active_pieces {
                if piece.square_name == name {
                    commands.entity(entity).insert(HighlightedPiece);
                    break;
                }
            }
        } else { // There was already a selected piece
            let (hl_entity, hl_piece, hl_transform) = hl_piece_query.single_mut();

            for (entity, piece, transform) in active_pieces {
                if piece.square_name == name {
                    // Change the selected piece to the new piece
                    // and return early
                    commands.entity(hl_entity).remove::<HighlightedPiece>();
                    commands.entity(entity).insert(HighlightedPiece);
                    return;
                }
            }

            // Generate current possible move squares and compare to
            // the clicked square

        }
    }
}