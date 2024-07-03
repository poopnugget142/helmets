use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use client::Predicted;
use input::MovementStateActions;
use lightyear::prelude::*;
use lightyear::utils::bevy_xpbd_2d::*;

use self::input::PlayerActions;

use super::*;

#[derive(Component, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Character;

// The player actively controlling the entity (This should probally be optional so we can have AI)
// I don't think this works tbh because of differences in entities between client and server
#[derive(Component, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PlayerOwner(pub Entity);

#[derive(Component)]
pub struct ParentSprite;

#[derive(Component, Deserialize, Serialize, Clone, Debug, PartialEq, Default, Reflect)]
pub enum MovementState {
    #[default]
    Walking,
    Running,
    SlowWalk,
    Crawling,
}

#[derive(Bundle)]
pub struct CharacterBundle {
    character: Character,
    position: Position,
    rotation: Rotation,
    physics: PhysicsBundle,
    movement_state: MovementState,
    transform: TransformBundle,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            character: Character,
            position: Position(Vec2::new(1.0, 1.0)),
            rotation: Rotation::ZERO,
            physics: PhysicsBundle::default(),
            movement_state: MovementState::default(),
            transform: TransformBundle::default(),
        }
    }
}

#[derive(Bundle)]
pub struct PhysicsBundle {
    velocity: LinearVelocity,
    angular_velocity: AngularVelocity,
    collider: Collider,
    rigid_body: RigidBody,
    damping: LinearDamping,
    angular_damping: AngularDamping,
}

impl Default for PhysicsBundle {
    fn default() -> Self {
        Self {
            velocity: LinearVelocity::default(),
            angular_velocity: AngularVelocity::default(),
            collider: Collider::rectangle(40.0, 40.0),
            rigid_body: RigidBody::Dynamic,
            damping: LinearDamping(5.0),
            angular_damping: AngularDamping(5.0)
        }
    }
}

pub(super) fn register(app: &mut App) {
    app.register_type::<MovementState>();

    app.register_component::<Character>(ChannelDirection::Bidirectional)
        .add_prediction(ComponentSyncMode::Once)
        .add_interpolation(ComponentSyncMode::Once);

    app.register_component::<Position>(ChannelDirection::Bidirectional)
        .add_prediction(ComponentSyncMode::Full)
        .add_interpolation(ComponentSyncMode::Full)
        .add_interpolation_fn(position::lerp)
        .add_correction_fn(position::lerp);

    app.register_component::<Rotation>(ChannelDirection::Bidirectional)
        .add_prediction(ComponentSyncMode::Full)
        .add_interpolation(ComponentSyncMode::Full)
        .add_interpolation_fn(rotation::lerp)
        .add_correction_fn(rotation::lerp);

    app.register_component::<Transform>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Once);

    app.register_component::<GlobalTransform>(ChannelDirection::ServerToClient)
        .add_prediction(ComponentSyncMode::Once);

    app.register_component::<MovementState>(ChannelDirection::Bidirectional)
        .add_prediction(ComponentSyncMode::Full);

    app.register_component::<LinearVelocity>(ChannelDirection::Bidirectional)
        .add_prediction(ComponentSyncMode::Full);

    app.register_component::<AngularVelocity>(ChannelDirection::Bidirectional)
        .add_prediction(ComponentSyncMode::Full);

    app.add_systems(Update, shared_turn_behavior);
    

    // app.register_component::<PlayerOwner>(ChannelDirection::Bidirectional)
    //     .add_prediction(ComponentSyncMode::Once)
    //     .add_interpolation(ComponentSyncMode::Once);
}

const MAX_VELOCITY: f32 = 200.0;

pub fn shared_movement_behaviour(
    mut velocity: Mut<LinearVelocity>,
    action: &ActionState<PlayerActions>,
) {
    const MOVE_SPEED: f32 = 5.0;
    if action.pressed(&PlayerActions::Up) {
        velocity.y += MOVE_SPEED;
    }
    if action.pressed(&PlayerActions::Down) {
        velocity.y -= MOVE_SPEED;
    }
    if action.pressed(&PlayerActions::Left) {
        velocity.x -= MOVE_SPEED;
    }
    if action.pressed(&PlayerActions::Right) {
        velocity.x += MOVE_SPEED;
    }
    *velocity = LinearVelocity(velocity.clamp_length_max(MAX_VELOCITY));
}

const TURN_SPEED: f32 = 7.5;
pub fn shared_turn_behavior(
    mut query: Query<
        (
            &Transform,
            &mut ActionState<PlayerActions>,
            &mut AngularVelocity,
        ),
        Or<(With<Predicted>, With<ReplicationTarget>)>,
    >,
) {
    for (transform, action, mut angular_velocity) in query.iter_mut() {
        let Some(cursor_data) = action.action_data(&PlayerActions::MoveCursor) else {
            continue;
        };

        let cursor_dir = cursor_data
            .axis_pair
            .map(|axis| axis.xy())
            .unwrap_or_default();

        dbg!(&cursor_dir);

        if transform.rotation.is_nan() {
            angular_velocity.0 = TURN_SPEED;
            continue;
        }

        // In 2d up is positive y, so current forward direction for the player
        // is local_y unit vector
        let player_dir = transform.local_y().truncate();
        
        // Then we find the angle between current forward direction and desired one
        let cursor_angle = player_dir.angle_between(cursor_dir);
        
        // And finally rotate player along z with that difference
        if cursor_angle.abs() < 0.01 {
            angular_velocity.0 = 0.0;
            continue;
        }

        angular_velocity.0 = (cursor_angle.cbrt() * TURN_SPEED).max(TURN_SPEED);
    }
}

pub fn shared_movement_state_behavior(
    mut movement_state: Mut<MovementState>,
    action: &ActionState<MovementStateActions>,
){
    if *movement_state == MovementState::Crawling {
        if action.just_pressed(&MovementStateActions::Crawl) || action.just_pressed(&MovementStateActions::Run) {
            *movement_state = MovementState::Walking;
            return;
        } else {
            return;
        }
    }

    if action.just_pressed(&MovementStateActions::Crawl) {
        *movement_state = MovementState::Crawling;
        return;
    }

    if action.pressed(&MovementStateActions::Run) {
        *movement_state = MovementState::Running;
        return;
    }

    if action.pressed(&MovementStateActions::SlowWalk) {
        *movement_state = MovementState::SlowWalk;
        return;
    }

    if *movement_state != MovementState::Walking {
        *movement_state = MovementState::Walking;
    }
}