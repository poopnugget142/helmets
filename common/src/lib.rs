// Generalized Lib

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};

use std::time::Duration;
use lightyear::{client::components::ComponentSyncMode, prelude::*};

pub const PORT: u16 = 32105;
pub const PROTOCOL_ID: u64 = 0;
pub const KEY: [u8; 32]  = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub const REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);
pub const FIXED_TIMESTEP_HZ: f64 = 64.0;

pub mod player;
pub mod character;
pub mod input;

pub fn shared_config(mode: Mode) -> SharedConfig {
    SharedConfig {
        // How often the client will send packets to the server (by default it is every frame).
        // Currently, the client only works if it sends packets every frame, for proper input handling.
        client_send_interval: Duration::default(),
        // How often the server will send packets to clients? You can reduce this to save bandwidth.
        server_send_interval: Duration::from_millis(40),
        // The tick rate that will be used for the FixedUpdate schedule
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / 64.0),
        },
        // Here we make the `Mode` an argument so that we can run `lightyear` either in `Separate` mode (distinct client and server apps)
        // or in `HostServer` mode (the server also acts as a client).
        mode,
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum FixedSet {
    // main fixed update systems (handle inputs)
    Main,
    // apply physics steps
    Physics,
}

#[derive(Channel)]
pub struct Channel1;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message1(pub usize);


pub fn register(app: &mut App) {
    player::register(app);
    character::register(app);
    input::register(app);

    // physics
    app.add_plugins(PhysicsPlugins::new(FixedUpdate))
        .insert_resource(Time::new_with(Physics::fixed_once_hz(FIXED_TIMESTEP_HZ)))
        .insert_resource(Gravity(Vec2::ZERO));

    app.configure_sets(
        FixedUpdate,
        (
            // make sure that any physics simulation happens after the Main SystemSet
            // (where we apply user's actions)
            (
                PhysicsSet::Prepare,
                PhysicsSet::StepSimulation,
                PhysicsSet::Sync,
            )
                .in_set(FixedSet::Physics),
            (FixedSet::Main, FixedSet::Physics).chain(),
        ),
    );

    // channels
    app.add_channel::<Channel1>(ChannelSettings {
        mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
        ..default()
    });

    // messages
    app.add_message::<Message1>(ChannelDirection::Bidirectional);
}