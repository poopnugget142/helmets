// Movement controller

use super::*;
use leafwing_input_manager::prelude::*;

// #[derive(Event)]
// pub struct UpdateCharacterTransform {
//     pub transform: CharacterTransform,
//     pub client_id: ClientId,
// }

// #[derive(Component, Default, Debug, PartialEq, Eq, Clone)]
// pub enum MovementState {
//     #[default]
//     Walking,
//     Running,
//     SlowWalk,
//     Crawling,
// }

pub(super) fn register(app: &mut App) {

    // .add_event::<UpdateCharacterTransform>()

    // .add_systems(Update, (character_movement, set_movement_state, update_other_transform).in_set(LocalCharacterSet))
    // .add_systems(Update, (update_server).in_set(ConnectedSet).in_set(LocalCharacterSet).run_if(on_real_timer(Duration::from_millis(50))));
}



// Mouse turning and movement should be 2 seperate functions this is hard to read
// fn character_movement(
//     mut q_character: Query<(&mut Transform, &mut LinearVelocity, &MovementState), With<LocalCharacter>>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     windows: Query<&mut Window>,
//     camera: Query<(&Camera, &GlobalTransform)>,
// ) {
//     let window = windows.single();
//     let (camera, camera_transform) = camera.single();
//     let (mut transform, mut linear_velocity, movement_state) = q_character.single_mut();

//     let mut movement_vector = Vec2::new(0.0, 0.0);

//     if keyboard_input.pressed(KeyCode::KeyW) {
//         movement_vector += Vec2::new(0.0, 1.0);
//     }
//     if keyboard_input.pressed(KeyCode::KeyS) {
//         movement_vector += Vec2::new(0.0, -1.0);
//     }
//     if keyboard_input.pressed(KeyCode::KeyD) {
//         movement_vector += Vec2::new(1.0, 0.0);
//     }
//     if keyboard_input.pressed(KeyCode::KeyA) {
//         movement_vector += Vec2::new(-1.0, 0.0);
//     }

//     let movement_force;
//     match movement_state {
//         MovementState::SlowWalk => {movement_force = 300.0}
//         MovementState::Walking => {movement_force = 600.0}
//         MovementState::Running => {movement_force = 1000.0}
//         MovementState::Crawling => {movement_force = 0.0}
//     }

//     //what
//     linear_velocity.0 = movement_vector*movement_force;

//     if let Some(cursor_position) = window.cursor_position()
//     .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
//         // In 2d up is positive y, so current forward direction for the player
//         // is local_y unit vector
//         let player_dir = transform.local_y().truncate();

//         // Direction to cursor (what we want local_y to become) is simply the
//         // difference of target position and player position
//         let cursor_dir = cursor_position - transform.translation.truncate();
        
//         // Then we find the angle between current forward direction and desired one
//         let angle = player_dir.angle_between(cursor_dir);
        
//         // And finally rotate player along z with that difference
//         transform.rotate_z(angle);
//     }
// }

// fn set_movement_state(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut q_character: Query<&mut MovementState, With<LocalCharacter>>,
// ) {
//     let mut movement_state = q_character.single_mut();

//     //not sure what the * does but it makes the code work so yay?
//     if *movement_state == MovementState::Crawling {

//         if keyboard_input.just_pressed(KeyCode::KeyC) || keyboard_input.just_pressed(KeyCode::ShiftLeft) {
//             *movement_state = MovementState::Walking;
//             return;
//         } else {
//             return;
//         }
//     }

//     if keyboard_input.just_pressed(KeyCode::KeyC) {
//         *movement_state = MovementState::Crawling;
//         return;
//     }

//     if keyboard_input.pressed(KeyCode::ShiftLeft) {
//         *movement_state = MovementState::Running;
//         return;
//     }

//     if keyboard_input.pressed(KeyCode::AltLeft) {
//         *movement_state = MovementState::SlowWalk;
//         return;
//     }

//     if *movement_state != MovementState::Walking {
//         *movement_state = MovementState::Walking;
//     }
// }

// fn update_other_transform (
//     mut r_character_transform: EventReader<UpdateCharacterTransform>,
//     q_controlled_character: Query<&ControlledCharacter>,
//     mut q_transform: Query<&mut Transform>,
//     player_list: Res<PlayerList>,
// ) {
//     for update_data in r_character_transform.read() {
//         let position = update_data.transform.position;
//         let rotation = update_data.transform.rotation;

//         let player = player_list.0.get(&update_data.client_id.to_string());

//         // Player does not exist yet move on
//         if let None = player { continue; }

//         let player = player.unwrap().to_owned();

//         let character = q_controlled_character.get(player);
        
//         // Character does not exist yet move on
//         if let Err(_) = character { continue; }

//         let character = character.unwrap().0;

//         let mut transform = q_transform.get_mut(character).unwrap();

//         transform.translation = Vec3::new(position.x, position.y, 0.0);
//         transform.rotation = Quat::from_rotation_z(rotation)
//     }
// }

// fn update_server (
//     characters: Query<(&Transform, &LocalCharacter)>,
//     mut client: ResMut<RenetClient>,
// ) {
//     let (transform, _) = characters.single();

//     let position = Vec2::new(transform.translation.x, transform.translation.y);
//     let rotation = transform.rotation.to_euler(EulerRot::ZYX).0;

//     let character_transform = CharacterTransform {
//         position,
//         rotation,
//     };

//     let client_message = ClientMessage::LocalMovement(character_transform);
//     let client_message = bincode::serialize(&client_message).unwrap();

//     client.send_message(DefaultChannel::ReliableOrdered, client_message);
// }