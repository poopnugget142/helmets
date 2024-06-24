//Generic character class code

use bevy_xpbd_2d::{components::{ColliderDensity, LinearVelocity, Position, RigidBody}, plugins::collision::Collider};

use self::input::PlayerActions;

use super::*;

#[derive(Bundle)]
pub struct CharacterBundle {
    character: Character,
    replicate: Replicate,
    pre_predicted: PrePredicted,
    collider: Collider,
    collider_density: ColliderDensity,
    friction: Friction,
    rigid_body: RigidBody,
    position: Position,
    parent_sprite: ParentSprite,
    sprite_bundle: SpriteBundle,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        let position = Vec2::new(0.0, 0.0);
        Self {
            character: Character,
            replicate: Replicate {
                prediction_target: NetworkTarget::All,
                replication_group: REPLICATION_GROUP,
                ..default()
            },
            pre_predicted: PrePredicted::default(),
            collider: Collider::rectangle(40.0, 40.0),
            collider_density: ColliderDensity(20.0),
            friction: Friction::new(0.5),
            rigid_body: RigidBody::Dynamic,
            position: Position(position),
            parent_sprite: ParentSprite,
            sprite_bundle: SpriteBundle::default(),
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
    // app.register_component::<Character>(ChannelDirection::ServerToClient)
    //     .add_prediction::<Character>(ComponentSyncMode::Once)
    //     .add_interpolation::<Character>(ComponentSyncMode::Once);

    // app.register_component::<PlayerOwner>(ChannelDirection::ServerToClient)
    //     .add_prediction::<PlayerOwner>(ComponentSyncMode::Once)
    //     .add_interpolation::<PlayerOwner>(ComponentSyncMode::Once);
}

const MAX_VELOCITY: f32 = 200.0;

pub fn shared_movement_behaviour(
    mut velocity: Mut<LinearVelocity>,
    action: &ActionState<PlayerActions>,
) {
    const MOVE_SPEED: f32 = 10.0;
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