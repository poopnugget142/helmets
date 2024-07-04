use leafwing_input_manager::axislike::DualAxisData;

// Movement controller
use super::*;

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, update_look_direction.in_set(LocalCharacterSet));
}

fn update_look_direction(
    mut character_q: Query<(&mut ActionState<PlayerActions>, &Transform), With<LocalCharacter>>,
    windows: Query<&mut Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera.single();
    let (mut action_state, transform) = character_q.single_mut();

    if let Some(cursor_position) = window.cursor_position()
    .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
        let cursor_dir = (cursor_position - transform.translation.truncate()).normalize_or_zero();

        action_state.press(&PlayerActions::MoveCursor);
        action_state
            .action_data_mut(&PlayerActions::MoveCursor)
            .unwrap()
            .axis_pair = Some(DualAxisData::from_xy(cursor_dir));
    }
}