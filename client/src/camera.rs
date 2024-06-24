// Camera controller that focuses on player/mouse

use bevy::prelude::*;

// use crate::character::{LocalCharacter, LocalCharacterSet};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup);
        // .add_systems(Update, focus_on_character.in_set(LocalCharacterSet));
    }
}

fn setup (mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
    ));
}

// fn focus_on_character (
//     mut set: ParamSet<(
//         Query<&Transform, With<LocalCharacter>>,
//         Query<&mut Transform, With<Camera>>,
//     )>,
// ) {
//     //wtf even is this //idk fix it
//     set.p1().single_mut().translation = set.p0().single().translation;
// }