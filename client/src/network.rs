// connects entities to one another using lightyear

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use client::{ClientConnection, Confirmed, Interpolated, InterpolationSet, Predicted, PredictionSet};
use common::{character::*, input::PlayerActions, player::PlayerId, *};
use leafwing_input_manager::{action_state::ActionState, input_map::InputMap, InputManagerBundle};
use lightyear::{client::events::ConnectEvent, prelude::{client::ClientCommands, *}, transport::config::SharedIoConfig};

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr}, time::SystemTime
};

// #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
// pub enum NetworkState {
//     #[default]
//     Disconnected,
//     Connecting,
//     Connected,
// }

#[derive(Resource)]
pub struct LocalClientId(pub u64);


#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConnectedSet;

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {

        println!("Client running!");
    
        let server_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), PORT);
    
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
    
        // Creates id for the server to identify the client as
        let client_id = current_time.as_millis() as u64;
    
        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
    
        let io_config = SharedIoConfig::from_transport(client::ClientTransport::UdpSocket(client_addr));
    
        let auth = client::Authentication::Manual {
            // server's IP address
            server_addr,
            // ID to uniquely identify the client
            client_id,
            // private key shared between the client and server
            private_key: KEY,
            // PROTOCOL_ID identifies the version of the protocol
            protocol_id: PROTOCOL_ID,
        };
    
        let net_config = client::NetConfig::Netcode {
            auth,
            io: io_config,
            config: client::NetcodeConfig::default(),
        };
    
        let client_config = client::ClientConfig {
            shared: shared_config(Mode::Separate),
            net: net_config,
            ..default()
        };
    
        let client_plugin = client::ClientPlugins::new(client_config);
    
        app.add_plugins(client_plugin);
        app.insert_resource(LocalClientId(client_id));
        app.add_systems(Update, handle_connection);
        app.add_systems(Startup, init);
        // draw after interpolation is done
        app.add_systems(
            PostUpdate,
            draw_elements
                .after(InterpolationSet::Interpolate)
                .after(PredictionSet::VisualCorrection),
        );
        // app.init_state::<NetworkState>();
        // app.add_systems(Update, (check_connection)
        //     .run_if(in_state(NetworkState::Connecting).or_else(in_state(NetworkState::Connected)) ));
    }
}

pub(crate) fn draw_elements(
    mut gizmos: Gizmos,
    players: Query<(&Position), (Without<Confirmed>, With<Character>)>,
) {
    for (position) in &players {
        gizmos.rect_2d(
            Vec2::new(position.x, position.y),
            Rotation::ZERO.as_radians(), //REPLACE WITH REAL ROTATION
            Vec2::ONE * 40.0,
            Color::rgb(100.0, 0.0, 0.0)
        );
    }
}

fn handle_connection(
    mut commands: Commands,
    mut connection_event: EventReader<ConnectEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in connection_event.read() {
        println!("CONNECTED");

        let client_id = event.client_id();

        let mut character = commands.spawn((
            PlayerId(client_id),
            CharacterBundle::default(),
            InputManagerBundle::<PlayerActions> {
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    (PlayerActions::Up, KeyCode::KeyW),
                    (PlayerActions::Down, KeyCode::KeyS),
                    (PlayerActions::Left, KeyCode::KeyA),
                    (PlayerActions::Right, KeyCode::KeyD),
                ]),
            },
        ));

        character.despawn_descendants();
            character.clear_children();

            character.with_children(|parent|{
                parent.spawn(SpriteBundle {
                    texture: asset_server.load("images/uniforms/french.png"),
                    transform: Transform {
                        ..default()
                    },
                    ..default()
                });
            });
    }
}

fn init(
    mut commands: Commands,
    client_id: Res<LocalClientId>,
) {
    commands.connect_client();

    commands.spawn(TextBundle::from_section(
        format!("CLIENT: {}", client_id.0),
        TextStyle {
            font_size: 20.0,
            color: Color::WHITE,
            ..default()
        },
    ));
}

// fn receive_message_system(
//     mut client: ResMut<RenetClient>,
//     mut new_chat_message: EventWriter<ChatMessage>,
//     mut create_player: EventWriter<CreatePlayer>,
//     mut new_character_transform: EventWriter<UpdateCharacterTransform>,
// ) {
//     while let Some(ser_message) = client.receive_message(DefaultChannel::ReliableOrdered) {
//         // Handle received message
//         let server_message: ServerMessage = bincode::deserialize(&ser_message).unwrap();
//         match server_message {
//             ServerMessage::Pong => {
//                 println!("Pong");
//             }
//             ServerMessage::BroadcastChat(client_id, text) => {
//                 new_chat_message.send(ChatMessage {
//                     client_id: Some(client_id),
//                     text,
//                 });
//             }
//             ServerMessage::PlayerData(player_data) => {
//                 create_player.send(CreatePlayer {
//                     player_data,
//                     local_player: false,
//                 });
//             }
//             ServerMessage::PlayerDisconnect(client_id, reason) => {
//                 new_chat_message.send(ChatMessage {
//                     client_id: None,
//                     text: format!("Client {client_id} disconnected: {reason}"),
//                 });
//             }
//             ServerMessage::Movement(character_transform, client_id) => {
//                 new_character_transform.send(UpdateCharacterTransform{
//                     transform: character_transform,
//                     client_id,
//                 });
//             }
//         }
//     }   
// }

// fn connect_window(
//     mut contexts: EguiContexts,
//     mut server_data: ResMut<ServerData>,
//     client: Option<ResMut<RenetClient>>,
//     network_state: Res<State<NetworkState>>,
//     mut next_network_state: ResMut<NextState<NetworkState>>,
// ) {
//     egui::Window::new("Server Data").show(contexts.ctx_mut(), |ui| {
//         ui.horizontal(|ui| {
//             ui.label("IPV4:");
//             ui.text_edit_singleline(&mut server_data.ip_address);
//         });
//         ui.horizontal(|ui| {
//             ui.label("PORT:");
//             ui.label(PORT.to_string());
//         });

//         ui.horizontal(|ui| {
//             ui.label("USERNAME:");
//             ui.text_edit_singleline(&mut server_data.user_name);
//             ui.label("COLOR:");
//             ui.color_edit_button_rgb(&mut server_data.color);
//         });

//         ui.horizontal(|ui| {
//             match network_state.get() {
//                 NetworkState::Connected => {
//                     ui.label("CONNECTED");
//                     let ping_responce = ui.button("PING");

//                     if let None = client {
//                         return;
//                     }

//                     let mut client = client.unwrap();

//                     // if ping_responce.clicked() {
//                     //     let ping_message = bincode::serialize(&ClientMessage::Ping).unwrap();
//                     //     client.send_message(DefaultChannel::ReliableOrdered, ping_message);
//                     //     println!("Ping");
//                     // }
//                 }
//                 NetworkState::Connecting => {
//                     ui.label("CONNECTING");
//                 }
//                 NetworkState::Disconnected => {
//                     let button_responce = ui.button("CONNECT");
//                     if button_responce.clicked() {
//                         next_network_state.set(NetworkState::Connecting);
//                     }
//                 }
//             }
//         })
//     });
// }
