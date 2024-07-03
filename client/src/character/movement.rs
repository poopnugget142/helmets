use leafwing_input_manager::axislike::DualAxisData;

// Movement controller
use super::*;

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, set_movement_state.in_set(LocalCharacterSet));
    app.add_systems(Update, player_movement);
    app.add_systems(Update, update_look_direction.in_set(LocalCharacterSet));
}

fn player_movement(
    tick_manager: Res<TickManager>,
    mut action_query: Query<
        (
            Entity,
            &Position,
            &mut LinearVelocity,
            &ActionState<PlayerActions>,
        ),
    >,
) {
    for (entity, position, velocity, action,) in action_query.iter_mut() {
        if !action.get_pressed().is_empty() {
            // NOTE: be careful to directly pass Mut<PlayerPosition>
            // getting a mutable reference triggers change detection, unless you use `as_deref_mut()`
            shared_movement_behaviour(velocity, action);
            trace!(?entity, tick = ?tick_manager.tick(), ?position, actions = ?action.get_pressed(), "applying movement to player");
        }
    }
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

fn set_movement_state(
    mut character_q: Query<(&mut MovementState, &ActionState<MovementStateActions>), With<LocalCharacter>>,
) {
    let (movement_state, action) = character_q.single_mut();
    shared_movement_state_behavior(movement_state, action)
}