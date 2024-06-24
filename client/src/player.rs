// Player Class

use bevy::prelude::*;
use common::player::{self, PlayerId};

#[derive(Component)]
pub struct ControlledCharacter (pub Entity);

#[derive(Component)]
pub struct LocalPlayer;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // let player_list = PlayerList (HashMap::new());

        // .insert_resource(player_list)

        // .add_event::<CreatePlayer>()

        // .add_systems(Update, created_player); 
    }
}

// #[derive(Event)]
// pub struct CreatePlayer {
//     // you can have some parameters
//     pub player_data: ServerPlayerData,
//     pub local_player: bool,
// }

// fn created_player (
//     mut commands: Commands,
//     mut create_player_reader: EventReader<CreatePlayer>,
//     mut send_chat: EventWriter<ChatMessage>,
//     mut player_list: ResMut<PlayerList>,
//     mut character_add_writer: EventWriter<CreateCharacter>,
// ) {

//     for event in create_player_reader.read() {
//         let player_data = &event.player_data;
//         let player = Player{
//             name: player_data.name.clone(),
//             client_id: player_data.client_id,
//             color: player_data.color,
//         };

//         let mut entity = commands.spawn_empty();
//         entity.insert(player);

//         if event.local_player {
//             entity.insert(LocalPlayer);
//         }

//         player_list.0.insert(event.player_data.client_id.to_string(), entity.id());

//         send_chat.send(ChatMessage {
//             client_id: None,
//             text: format!("{} has connected to the game", player_data.name)
//         });

//         character_add_writer.send(CreateCharacter {
//             local_character: event.local_player,
//             player: entity.id(),
//         });
//     }
// }

// pub struct RemovePlayer {
//     // you can have some parameters
//     pub client_id: ClientId,
//     pub reason: String,
// }

// impl Command for RemovePlayer {
//     fn apply(self, world: &mut World) {
//         let mut player_list = world.resource_mut::<PlayerList>();

//         let client_id = self.client_id;

//         //get player
//         let player = player_list.0.get(&client_id.to_string()).unwrap().to_owned();

//         //remove from player list
//         player_list.0.remove(&client_id.to_string());

//         //delete entity
//         world.get_entity_mut(player).unwrap().despawn();
//     }
// }