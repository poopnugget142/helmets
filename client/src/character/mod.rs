// Base code for characters

use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use client::{Confirmed, Predicted};
use common::*;
use character::*;
use input::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

// use self::movement::MovementState;

pub mod movement;
pub mod appearance;

#[derive(Component)]
pub struct LocalCharacter;

#[derive(Component)]
struct Footsteps;

#[derive(Component)]
struct Dead;

#[derive(Component)]
struct Temp {
    pub time_alive: Duration,
    pub time_started: Instant,
}

#[derive(Event)]
pub struct CreateCharacter {
    pub local_character: bool,
    pub player: Entity,
}

fn is_local_character(
    q_characters: Query<&LocalCharacter>,
) -> bool {
    !q_characters.is_empty()
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalCharacterSet;

pub(super) fn register(app: &mut App) {
    app.configure_sets(Update, LocalCharacterSet.run_if(is_local_character));

    // .add_event::<CreateCharacter>()

    // .add_systems(Update, (create_character, delete_temp));
    //.add_systems(Update, (shoot).in_set(LocalCharacterSet));

    // app.add_systems(Update, other_character_created.before(PhysicsSet::Prepare));

    movement::register(app);
    appearance::register(app);
}

// fn create_character (
//     mut commands: Commands,
//     mut character_add_reader: EventReader<CreateCharacter>,
// ) {
//     for character_data in character_add_reader.read() {

//         let mut character = commands.spawn((
//         ParentSprite,
//         SpatialBundle::default(),
//         SpatialListener::new(400.0),
//         Collider::circle(30.0),
//         RigidBody::Dynamic,
//         GravityScale(0.0),
//         //ColliderMassProperties::Density(2.0),
//         // ExternalForce {
//         //     force: Vec2::new(0.0, 0.0),
//         //     torque: 0.0,
//         // },
//         // Damping { linear_damping: 6.0, angular_damping: 0.0 },
//         LockedAxes::ROTATION_LOCKED,
//         MovementState::Walking,
//         ));

//         if character_data.local_character {
//             character.insert(LocalCharacter);
//         }

//         let character_id = character.id();

//         let mut player = commands.entity(character_data.player);

//         player.insert(ControlledCharacter(character_id));
//     }
// }

// fn delete_temp (
//     mut commands: Commands,
//     q_temp: Query<(&Temp, Entity)>,
// ) {
//     for (temp, entity) in q_temp.iter() {
//         if temp.time_alive < Instant::now().duration_since(temp.time_started) {
//             let mut entity_commands = commands.entity(entity);
//             entity_commands.despawn();
//         }
//     }
// }

// fn shoot(
//     mut commands: Commands,
//     camera: Query<(&Camera, &GlobalTransform)>,
//     mouse: Res<ButtonInput<MouseButton>>,
//     asset_server: Res<AssetServer>,
//     q_character: Query<(&Transform, Entity), With<LocalCharacter>>,
//     q_window: Query<&mut Window>,
//     rapier_context: Res<RapierContext>
// ) {
//     if mouse.just_pressed(MouseButton::Left) {
//         // Left button was pressed

//         // Does this get deleted? if not delete later or use the same audio track.
//         commands.spawn(AudioBundle {
//             source: asset_server.load("sounds/m1_garand_fire.mp3"),
//             settings: PlaybackSettings::DESPAWN,
//             ..default()
//         });

//         let (camera, camera_transform) = camera.single();

//         let (transform, character) = q_character.single();

//         let char_pos = transform.translation.truncate();

//         let window = q_window.single();

//         if let None = window.cursor_position() { return; }
//         // This works for now but in the future lets shoot in the direction the player is looking that way we can have slow turning weapons
//         let cursor_position = window.cursor_position().unwrap();

//         let cursor = camera.viewport_to_world(camera_transform, cursor_position).unwrap();

//         let cursor_dir = (cursor.origin.truncate() - char_pos).normalize()*1000.0;

//         let ray_pos = char_pos;
//         let ray_dir = cursor_dir;
//         let max_toi = 4.0;
//         let solid = true;
//         let filter = QueryFilter::default().exclude_rigid_body(character);

//         if let Some((entity, toi)) = rapier_context.cast_ray(
//             ray_pos, ray_dir, max_toi, solid, filter,
//         ) {
//             // The first collider hit has the entity `entity` and it hit after
//             // the ray travelled a distance equal to `ray_dir * toi`.
//             let hit_point = ray_pos + ray_dir * toi;

//             println!("Entity {:?} hit at point {}", entity, hit_point);

//             // Kill guy you shot
//             let mut entity_commanconfirmed: Query<(Entity, &PlayerId), Added<Predicted>>,ds = commands.entity(entity);
//             entity_commands.insert(Dead);

//             // commands.spawn((SpriteBundle {
//             //     sprite: Sprite {
//             //         custom_size: Some(Vec2::new(10.0, 10.0)),
//             //         color: Color::RED,
//             //         ..default()
//             //     },
//             //     transform: Transform::from_translation(hit_point.extend(0.0)),
//             //     ..default()
//             // },
//             // Temp {
//             //     time_alive: Duration::from_secs(3),
//             //     time_started: Instant::now(),
//             // }
//             // ));
//         }
//     }
// }