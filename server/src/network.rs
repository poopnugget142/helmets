// Network to Clients

use bevy::{ecs::entity, prelude::*};
use common::{character::{CharacterBundle, PlayerOwner}, input::PlayerActions, player::{PlayerBundle, PlayerId}, *};
use leafwing_input_manager::prelude::*;
use lightyear::{prelude::{server::ServerCommands, *}, server::{config::{NetcodeConfig, ServerConfig}, events::ConnectEvent, replication::{ReplicationConfig, ServerReplicationSet}}, shared::events::components::ComponentInsertEvent};

use std::net::{Ipv4Addr, SocketAddr};

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        println!("Server running!");

        let server_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), PORT);

        let netcode_config = NetcodeConfig {
            protocol_id: PROTOCOL_ID,
            private_key: Some(KEY.into()),
            ..default()
        };

        let net_config = server::NetConfig::Netcode {
            config: netcode_config,
            io: IoConfig::from_transport(TransportConfig::UdpSocket(server_addr)),
        };

        let replication = ReplicationConfig {
            enable_send: true,
            ..default()
        };

        let server_config = ServerConfig {
            shared: shared_config(Mode::Separate),
            // Here we only provide a single net config, but you can provide multiple!
            net: vec![net_config],
            replication,
            ..default()
        };

        let server_plugin = server::ServerPlugin::new(server_config);

        app.add_plugins(server_plugin);
        app.add_systems(Startup, init);
        // app.add_systems(Update, movement);
        app.add_systems(PreUpdate, replicate_players.in_set(ServerReplicationSet::ClientReplication));
    }
}

fn init(mut commands: Commands) {
    commands.start_server();
}

pub(crate) fn movement(
    mut action_query: Query<
        (
            &ActionState<PlayerActions>,
        ),
    >,
) {
    for action in action_query.iter_mut() {
        dbg!(action.0.get_pressed());
    }
}

// Replicate the pre-spawned entities back to the client
fn replicate_players(
    // mut player_spawn_reader: EventReader<ComponentInsertEvent<PlayerId>>,
    q_player_id: Query<(&PlayerId, Entity), Added<PlayerId>>,
    mut commands: Commands,
) {
    for (player_id, entity) in q_player_id.iter() {
        let client_id = player_id.0;

        // for all cursors we have received, add a Replicate component so that we can start replicating it
        // to other clients
        if let Some(mut e) = commands.get_entity(entity) {
            let mut replicate = Replicate {
                // we want to replicate back to the original client, since they are using a pre-predicted entity
                replication_target: NetworkTarget::All,
                ..default()
            };
            // We don't want to replicate the ActionState to the original client, since they are updating it with
            // their own inputs (if you replicate it to the original client, it will be added on the Confirmed entity,
            // which will keep syncing it to the Predicted entity because the ActionState gets updated every tick)!
            replicate.add_target::<ActionState<PlayerActions>>(NetworkTarget::AllExceptSingle(
                client_id,
            ));
            // if we receive a pre-predicted entity, only send the prepredicted component back
            // to the original client
            replicate.add_target::<PrePredicted>(NetworkTarget::Single(client_id));

            let predict_all = true;
            if predict_all {
                replicate.prediction_target = NetworkTarget::All;
                // // if we predict other players, we need to replicate their actions to all clients other than the original one
                // // (the original client will apply the actions locally)
                // replicate.disable_replicate_once::<ActionState<PlayerActions>>();
            } else {
                // we want the other clients to apply interpolation for the player
                replicate.interpolation_target = NetworkTarget::AllExceptSingle(client_id);
            }
            e.insert((
                replicate,
                // not all physics components are replicated over the network, so add them on the server as well
                // PhysicsBundle::player(),
            ));
        }
    }
}


// fn handle_connections(
//     // Here we listen for the `ConnectEvent` event
//     mut connections: EventReader<ConnectEvent>,
//     mut commands: Commands,
// ) {
//     for connection in connections.read() {
//         // on the server, the `context()` method returns the `ClientId` of the client that connected
//         let client_id = *connection.context();
        
//         // We add the `Replicate` component to start replicating the entity to clients
//         // By default, the entity will be replicated to all clients
//         let replicate = Replicate::default();
//         let player = commands.spawn((
//         PlayerBundle {
//             id: PlayerId(client_id),
//         },
//         replicate,
//         ));

//         let player_id = player.id();
        
//         // Add a mapping from client id to entity id
//         //global.client_id_to_entity_id.insert(client_id, entity.id());
//     }
// }