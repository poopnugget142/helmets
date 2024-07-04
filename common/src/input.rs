// Input actions that the player can take

use super::*;

/// The different directions that the player can move the character
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash, Reflect, Actionlike)]
pub enum PlayerActions {
    Up,
    Down,
    Left,
    Right,
    MoveCursor,
    Run,
    SlowWalk,
    Crawl,
    Shoot,
}

pub(super) fn register(app: &mut App) {
    // Register inputs
    app.add_plugins(LeafwingInputPlugin::<PlayerActions>::default());
}