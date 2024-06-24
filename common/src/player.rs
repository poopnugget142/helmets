// Generic Player Class

use super::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub id: PlayerId,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct PlayerId(pub ClientId);

pub(super) fn register(app: &mut App) {
    app.register_type::<PlayerId>();

    app.register_component::<PlayerId>(ChannelDirection::Bidirectional)
        .add_prediction::<PlayerId>(ComponentSyncMode::Once)
        .add_interpolation::<PlayerId>(ComponentSyncMode::Once);
}