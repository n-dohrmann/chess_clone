
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::constants::*;
use crate::pieces::*;

pub fn print_pos_when_clicked(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            let world_position = cursor_to_world_position(position);
            println!("Mouse clicked at {:?}", world_position);
        }
    }
}
/// Translate a cursor position into a world position.
pub fn cursor_to_world_position(cursor_position: Vec2) -> Vec2 {
    Vec2::new(cursor_position.x, WINDOW_SIZE - cursor_position.y)
}
