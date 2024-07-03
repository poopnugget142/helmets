// Code that renders the correct images onto the players depending on their current states.
// A sprite has multiple animated images layered on top of it

use super::*;

pub(super) fn register(app: &mut App) {
    app
    .add_systems(Update, update_appearance);
}

fn update_appearance (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    character_q: Query<(Entity, &MovementState), Or<(Changed<MovementState>, Added<Dead>)>>,
    dead_q: Query<&Dead>,
) {
    for (entity, movement_state) in character_q.iter() {
        let mut character = commands.entity(entity);

        if let Ok(_) = dead_q.get(entity) {
            println!("I have died");

            character.despawn_descendants();
            character.clear_children();

            character.with_children(|parent|{
                parent.spawn(SpriteBundle {
                    texture: asset_server.load("images/uniforms/french-dead.png"),
                    transform: Transform {
                        ..default()
                    },
                    ..default()
                });
            });

            continue;
        }
    
        match movement_state {
            MovementState::Crawling => {
                character.despawn_descendants();
                character.clear_children();

                character.with_children(|parent|{
                    parent.spawn(SpriteBundle {
                        texture: asset_server.load("images/uniforms/french-lay-aim.png"),
                        transform: Transform {
                            ..default()
                        },
                        ..default()
                    });
                });
            }
    
            MovementState::Walking => {
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
            
                    parent.spawn(SpriteBundle {
                        texture: asset_server.load("images/guns/idle/rifle.png"),
                        transform: Transform {
                            ..default()
                        },
                        ..default()
                    });
                });
            }
    
            _ => {}
        }

    }
}