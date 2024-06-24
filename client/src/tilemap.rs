// Render the tile map
// Probally scrap in future

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct TilemapPlugin;
impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
        .add_systems(Startup, setup);
    }
}

fn setup (mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        // Loads the tilemap into the world
        // TODO! weird sizing fix later
        ldtk_handle: asset_server.load("map1.ldtk"),
        
        ..Default::default()
    });
}