// Initialize the client code

use bevy::prelude::*;
use common;

use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod network;
// mod chat;
mod player;
mod character;
mod camera;
mod tilemap;

fn main() {
    // Creates the app
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Dawn of A Century".into(),
                    resolution: (640.0, 480.0).into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .build(),
        EguiPlugin,
        WorldInspectorPlugin::new(),
    ))
    .add_plugins((
        network::NetworkPlugin,
        // chat::ChatPlugin,
        player::PlayerPlugin,
        camera::CameraPlugin,
        tilemap::TilemapPlugin,
    ));

    //Ok yeah this is really cool
    common::register(&mut app);
    character::register(&mut app);

    // I can't believe you forgot this
    app.run();
}