

use bevy::prelude::*;
use crate::pieces::*;
use crate::constants::*;


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
