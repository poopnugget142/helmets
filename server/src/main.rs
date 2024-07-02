// Start server

use bevy::prelude::*;

use bevy::log::LogPlugin;

use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use common;

mod network;

fn main() {
    // Creates the app
    let mut app = App::new();

    app.add_plugins((
        MinimalPlugins,
        LogPlugin::default()
    ));
    // app.add_plugins((
    //     DefaultPlugins
    //         .set(WindowPlugin {
    //             primary_window: Some(Window {
    //                 title: "Dawn of A Century".into(),
    //                 resolution: (640.0, 480.0).into(),
    //                 resizable: true,
    //                 ..default()
    //             }),
    //             ..default()
    //         })
    //         .set(ImagePlugin::default_nearest())
    //         .build(),
    //     EguiPlugin,
    //     WorldInspectorPlugin::new(),
    // ));
    app.add_plugins((
        network::NetworkPlugin,
    ));

    common::register(&mut app);

    app.run();
}