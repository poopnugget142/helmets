// Start server

use bevy::prelude::*;

use bevy::log::LogPlugin;

use common;

mod network;

fn main() {
    // Creates the app
    let mut app = App::new();

    app.add_plugins((
        MinimalPlugins,
        LogPlugin::default()
    ))
    .add_plugins((
        network::NetworkPlugin,
    ));

    common::register(&mut app);

    app.run();
}