
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::constants::*;
use crate::board::*;
use bevy::color::palettes::basic::RED;


pub fn print_name_when_clicked(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    bins: Res<Bins>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            let piece_position = cursor_to_piece_position(position);
            let window_position = piece_to_window_position(piece_position);
            let clamped_coords = coordinate_clamp(window_position, &bins);
            let (rank, file) = get_rkfl_from_coords(clamped_coords);
            let name = get_square_name(rank, file).unwrap();
            println!("Clicked: {}", name);
        }
    }
}

// Get the chess board name of the clicked square.
pub fn get_name_clicked_square(
    cursor_position: Vec2,
    bins: &Res<Bins>,
) -> String {
    let piece_position = cursor_to_piece_position(cursor_position);
    let window_position = piece_to_window_position(piece_position);
    let clamped_coords = coordinate_clamp(window_position, bins);
    let (rank, file) = get_rkfl_from_coords(clamped_coords);
    get_square_name(rank, file).unwrap()
}

pub fn square_selection_logic(
    query: Query<(Entity, &Square, &mut Handle<ColorMaterial>), Without<HighlightedSquare>>,
    mut hs_query: Query<(Entity, &Square, &mut Handle<ColorMaterial>), With<HighlightedSquare>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    bins: Res<Bins>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            let name = get_name_clicked_square(position, &bins);

            if hs_query.is_empty() { // no square is highlighted
                for (entity, square, handle) in query.iter() {
                    if square.name == name {
                        let color_mat = materials.get_mut(handle).unwrap();
                        color_mat.color = Color::from(RED);
                        commands.entity(entity).insert(HighlightedSquare);
                        break;
                    }
                }
            } else { // There is already a highlighted square
                let (hs_entity, hs_square, hs_handle) = hs_query.single_mut();

                // Is the selected square different from the highlighted square?
                let mut new_square = false;

                for (entity, square, handle) in query.iter() {
                    if square.name == name {
                        let color_mat = materials.get_mut(handle).unwrap();
                        color_mat.color = Color::from(RED);
                        let color_mat_hs = materials
                            .get_mut(hs_handle.as_ref())
                            .unwrap();
                        color_mat_hs.color = hs_square.color;
                        commands.entity(hs_entity).remove::<HighlightedSquare>();
                        commands.entity(entity).insert(HighlightedSquare);
                        new_square = true;
                        break;
                        }
                }
                if !new_square { // Turn off the highlighted square
                    let color_mat_hs = materials
                        .get_mut(hs_handle.as_ref())
                        .unwrap();
                    color_mat_hs.color = Color::from(hs_square.color);
                    commands.entity(hs_entity).remove::<HighlightedSquare>(); 
                }
            }
        }
    }
}

/// Translate a cursor position into a piece position.
pub fn cursor_to_piece_position(cursor_position: Vec2) -> Vec2 {
    Vec2::new(cursor_position.x, WINDOW_SIZE - cursor_position.y)
}

/// Translate a piece position to a world position.
pub fn piece_to_window_position(piece_position: Vec2) -> Vec3 {
    Vec3::new(
        piece_position.x - DISPLACEMENT,
        piece_position.y - DISPLACEMENT,
        1.0,
    )
}

// clamp the position into the nearest multiple of 50
pub fn coordinate_clamp(raw_position: Vec3, bins: &Res<Bins>) -> Vec3 {
    let mut adj_x = raw_position.x;
    let mut adj_y = raw_position.y;
    
    for bin in &bins.0[..] {
        if adj_x < *bin {
            adj_x = *bin - 100.;
            break;
        }
    }
    if adj_x > *bins.0.last().unwrap() {
        adj_x = *bins.0.last().unwrap();
    }

    for bin in &bins.0[..] {
        if adj_y < *bin {
            adj_y = *bin - 100.;
            break;
        }
    }
    if adj_y > *bins.0.last().unwrap() {
        adj_y = *bins.0.last().unwrap();
    }

    Vec3::new(adj_x, adj_y, 1.0)
}
