// Allow players to send messages through chat

use bevy::prelude::*;
use bevy_egui::{egui::{self, Color32, RichText}, EguiContexts};
use bevy_renet::renet::{ClientId, DefaultChannel, RenetClient};
use common::*;

use crate::{network::{ConnectedSet, ServerData}, player::Player};
use common::PlayerList;

#[derive(Event, Clone)]
pub struct ChatMessage {
    pub client_id: Option<ClientId>,
    pub text: String,
}

#[derive(Default, Resource)]
struct ChatData {
    current_message: String,
    history: Vec<ChatMessage>,
}

pub struct ChatPlugin;
impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        let chat_data = ChatData {
            current_message: String::new(),
            history: Vec::new(),
        };

        app
        .insert_resource(chat_data)

        .add_event::<ChatMessage>()

        .add_systems(Update, (chat_window, new_chat_message).in_set(ConnectedSet));
    }
}

fn chat_window (
    mut contexts: EguiContexts,
    mut chat_data: ResMut<ChatData>,
    mut client: ResMut<RenetClient>,
    server_data: Res<ServerData>,
    player_list: Res<PlayerList>,
    player_query: Query<&Player>,
) {
    egui::Window::new("Chat").show(contexts.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for message in &chat_data.history {

                //Message sent from server
                if let None = message.client_id {
                    ui.label(&message.text);
                    continue;
                }

                let player = player_list.0.get(&message.client_id.unwrap().to_string());
                
                if let None = player {
                    println!("\x1b[93;1mPLAYER DOES NOT EXIST ON CLIENT PLEASE UPDATE\x1b[0m");
                    continue;
                };

                let player = player.unwrap();

                let player_data = player_query.get(player.to_owned()).unwrap();
                let c = player_data.color;
                let text_color = Color32::from_rgb((c[0]*255.0) as u8, (c[1]*255.0) as u8, (c[2]*255.0) as u8);
                //dbg!(text_color);

                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("[{}]:", player_data.name)).strong().color(text_color));
                    ui.label(&message.text);
                });
            }

            let response = ui.text_edit_singleline(&mut chat_data.current_message);

            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let server_message = bincode::serialize(&ClientMessage::PlayerChat(chat_data.current_message.clone())).unwrap();
                client.send_message(DefaultChannel::ReliableOrdered, server_message);

                let my_chat_message = ChatMessage {
                    client_id: server_data.client_id,
                    text: chat_data.current_message.clone(),
                };

                chat_data.history.push(my_chat_message);

                chat_data.current_message = "".to_string();
            }
        })
    });
}

fn new_chat_message (
    mut chat_reader: EventReader<ChatMessage>,
    mut chat_data: ResMut<ChatData>,
) {
    for chat_message in chat_reader.read() {
        chat_data.history.push(chat_message.clone())
    }
}
