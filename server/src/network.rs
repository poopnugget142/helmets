// Network to Clients

use bevy::prelude::*;
use input::MovementStateActions;
use leafwing_input_manager::prelude::*;
use bevy_xpbd_2d::prelude::*;
use lightyear::prelude::server::*;
use character::{shared_movement_behaviour, shared_movement_state_behavior, CharacterBundle, MovementState};
use common::{input::PlayerActions, player::PlayerId, *};
use lightyear::{prelude::*, transport::config::SharedIoConfig};
use std::net::{Ipv4Addr, SocketAddr};

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        println!("Server running!");

        let server_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), PORT);

        let netcode_config = NetcodeConfig {
            protocol_id: PROTOCOL_ID,
            private_key: KEY.into(),
            ..default()
        };

        let net_config = server::NetConfig::Netcode {
            config: netcode_config,
            io: SharedIoConfig::from_transport(server::ServerTransport::UdpSocket(server_addr))
        };

        let server_config = ServerConfig {
            shared: shared_config(Mode::Separate),
            // Here we only provide a single net config, but you can provide multiple!
            net: vec![net_config],
            ..default()
        };

        let server_plugin = server::ServerPlugins::new(server_config);

        app.add_plugins(server_plugin);
        app.add_systems(Startup, init);
        app.add_systems(Update, (movement, movement_state));
        app.add_systems(Update, handle_connection);
    }
}

fn init(mut commands: Commands) {
    commands.start_server();
}

fn handle_connection(
    mut commands: Commands,
    mut connection_event: EventReader<ConnectEvent>,
) {
    for event in connection_event.read() {
        let client_id = event.client_id;

        let mut sync_target = SyncTarget::default();

        // Can be set to all to predict every single client
        sync_target.prediction = NetworkTarget::Single(client_id);

        commands.spawn((
            PlayerId(client_id),
            CharacterBundle::default(),
            ActionState::<PlayerActions>::default(),
            ActionState::<MovementStateActions>::default(),
            server::Replicate {
                group: REPLICATION_GROUP,
                controlled_by: ControlledBy {
                    target: NetworkTarget::Single(client_id),
                },
                sync: sync_target,
                ..default()
            },
        ));
    }
}

/// Read client inputs and move players
/// NOTE: this system can now be run in both client/server!
fn movement(
    tick_manager: Res<TickManager>,
    mut action_query: Query<
        (
            Entity,
            &Position,
            &mut LinearVelocity,
            &ActionState<PlayerActions>,
        ),
    >,
) {
    for (entity, position, velocity, action) in action_query.iter_mut() {
        if !action.get_pressed().is_empty() {
            // NOTE: be careful to directly pass Mut<PlayerPosition>
            // getting a mutable reference triggers change detection, unless you use `as_deref_mut()`
            shared_movement_behaviour(velocity, action);
            trace!(?entity, tick = ?tick_manager.tick(), ?position, actions = ?action.get_pressed(), "applying movement to player");
        }
    }
}

fn movement_state(
    mut action_query: Query<
        (
            &mut MovementState,
            &ActionState<MovementStateActions>,
        ),
    >,
) {
    for (state, action) in action_query.iter_mut() {
        shared_movement_state_behavior(state, action);
    }
}