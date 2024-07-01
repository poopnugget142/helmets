//Generic character class code

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use lightyear::prelude::*;
use lightyear::utils::bevy_xpbd_2d::*;

use self::input::PlayerActions;

use super::*;

#[derive(Bundle)]
pub struct CharacterBundle {
    character: Character,
    replicate: client::Replicate,
    pre_predicted: PrePredicted,
    position: Position,
    parent_sprite: ParentSprite,
    sprite_bundle: SpriteBundle,
    physics: PhysicsBundle,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        let position = Vec2::new(0.0, 0.0);
        Self {
            character: Character,
            replicate: client::Replicate {
                group: REPLICATION_GROUP,
                ..default()
            },
            pre_predicted: PrePredicted::default(),
            position: Position(position),
            parent_sprite: ParentSprite,
            sprite_bundle: SpriteBundle::default(),
            physics: PhysicsBundle::default(),
        }
    }
}

#[derive(Bundle)]
pub struct PhysicsBundle {
    collider: Collider,
    collider_density: ColliderDensity,
    friction: Friction,
    rigid_body: RigidBody,
}

impl Default for PhysicsBundle {
    fn default() -> Self {
        Self {
            collider: Collider::rectangle(40.0, 40.0),
            collider_density: ColliderDensity(1000.0),
            friction: Friction::new(20.0),
            rigid_body: RigidBody::Dynamic,
        }
    }
}

#[derive(Component, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Character;

// The player actively controlling the entity (This should probally be optional so we can have AI)
#[derive(Component, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PlayerOwner(pub Entity);

#[derive(Component)]
struct ParentSprite;

pub(super) fn register(app: &mut App) {
    app.register_component::<Character>(ChannelDirection::Bidirectional)
        .add_prediction(ComponentSyncMode::Once)
        .add_interpolation(ComponentSyncMode::Once);

    app.register_component::<Position>(ChannelDirection::Bidirectional)
        .add_prediction(ComponentSyncMode::Full)
        .add_interpolation(ComponentSyncMode::Full)
        .add_interpolation_fn(position::lerp)
        .add_correction_fn(position::lerp);

    app.register_component::<LinearVelocity>(ChannelDirection::Bidirectional)
        .add_prediction(ComponentSyncMode::Full);

    app.register_component::<AngularVelocity>(ChannelDirection::Bidirectional)
        .add_prediction(ComponentSyncMode::Full);

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