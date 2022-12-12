use bevy::prelude::*;
use bevy_kira_audio::{DynamicAudioChannel, DynamicAudioChannels};
use bevy_rapier3d::prelude::CollisionEvent;
use serde::{Deserialize, Serialize};

use crate::{
    audio::play_sfx,
    bomb::Bomb,
    constants::SFX_AUDIO_CHANNEL,
    logic::Flag,
    map::Breakable,
    player::Player,
    utils::{animate_interactive_items, possibly_spawn_upgrade},
    GameState,
};
#[derive(Component, Clone, Debug, Serialize, Deserialize, PartialEq, Copy)]
pub enum UpgradeType {
    Bomb,
    Fire,
    Speed,
    None,
}
#[derive(Component)]
pub struct InteractiveItem {
    pub upgrade: UpgradeType,
}
pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(player_and_item_collision_listener)
                .with_system(player_and_flag_collision_listener)
                .with_system(animate_interactive_items)
                .with_system(explosion_collision_listener),
        );
    }
}

/// "When a collision between the player and an interactive item starts, despawn the item, play a sound
/// and give the player an upgrade."
///
/// The first thing we do is to get the collision events from the event reader. Then we iterate over
/// them
///
/// Arguments:
///
/// * `collision_events`: EventReader<CollisionEvent>
/// * `player_query`: Query<(Entity, &mut Player), With<Player>>,
/// * `interactive_query`: Query<(Entity, &InteractiveItem), Without<Player>>,
/// * `commands`: Commands - This is a struct that allows you to add, remove, and modify entities.
/// * `asset_server`: Res<AssetServer> - This is the asset server, which is used to load assets.
/// * `audio`: ResMut<DynamicAudioChannels>
pub fn player_and_item_collision_listener(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut Player), With<Player>>,
    interactive_query: Query<(Entity, &InteractiveItem), Without<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut audio: ResMut<DynamicAudioChannels>,
) {
    //Iterate over collision events
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_1, entity_2, _flags) => {
                //If found an event, check if envolves the player
                let (player_entity, mut player) = player_query.single_mut();
                let has_player_collide = player_entity == *entity_1 || player_entity == *entity_2;

                //If event is not related to player, ignore it, another
                //listener should handle it
                if !has_player_collide {
                    break;
                }
                let item_entity = if player_entity == *entity_1 {
                    entity_2
                } else {
                    entity_1
                };

                let is_interactive_item = interactive_query.contains(*item_entity);

                if !is_interactive_item {
                    break;
                }

                // Despawn item and play sound
                item_collision(
                    &mut commands,
                    item_entity.to_owned(),
                    asset_server.to_owned(),
                    audio.create_channel(SFX_AUDIO_CHANNEL),
                    String::from("audios/sfx/get_item.ogg"),
                );
                //Give Player Upgrade
                let (_entidade, item) = interactive_query.get(*item_entity).unwrap();
                if item.upgrade == UpgradeType::Bomb {
                    player.bomb_amount += 1; //Give the player more bombs
                } else if item.upgrade == UpgradeType::Fire {
                    player.bomb_range += 1.0; //Bombs affects 1.0 more on explosion
                } else {
                    player.speed += 0.2; //Player moves 0.2 times faster
                }
            }
            CollisionEvent::Stopped(_e1, _e2, _flags) => {
                // Collision OUT
            }
        }
    }
}

/// It listens for collision events between bombs and breakables, and if it finds one, it destroys the
/// breakable and spawns an item
///
/// Arguments:
///
/// * `collision_events`: EventReader<CollisionEvent>
/// * `bomb_query`: Query<Entity, With<Bomb>>,
/// * `breakable_query`: Query<(Entity, &Breakable, &Transform), Without<Bomb>>,
/// * `commands`: Commands - This is the command buffer that we will use to spawn new entities.
/// * `asset_server`: Res<AssetServer> - This is the asset server, which is used to load assets.
/// * `audio`: ResMut<DynamicAudioChannels>
pub fn explosion_collision_listener(
    mut collision_events: EventReader<CollisionEvent>,
    bomb_query: Query<Entity, With<Bomb>>,
    breakable_query: Query<(Entity, &Breakable, &Transform), Without<Bomb>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut audio: ResMut<DynamicAudioChannels>,
) {
    //Iterate over collision events
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_1, entity_2, _flags) => {
                //If found an event, check if envolves the bomb explosion
                let has_bomb_collide =
                    bomb_query.contains(*entity_1) || bomb_query.contains(*entity_2);

                //If found an event, check if envolves the breakable
                let has_breakable_collide =
                    breakable_query.contains(*entity_1) || breakable_query.contains(*entity_2);

                //If event is not related to bomb and breakables, ignore it, another
                //listener should handle it
                if !has_bomb_collide || !has_breakable_collide {
                    break;
                }

                //Get the breakable entity
                let breakable_entity = if breakable_query.contains(*entity_1) {
                    entity_1
                } else {
                    entity_2
                };
                let (_, _breakable, breakable_transform) =
                    breakable_query.get(*breakable_entity).unwrap();

                // Despawn breakable and play explosion sound
                item_collision(
                    &mut commands,
                    breakable_entity.to_owned(),
                    asset_server.to_owned(),
                    audio.create_channel(SFX_AUDIO_CHANNEL),
                    String::from("audios/sfx/bomb_explosion.ogg"),
                );
                //May or May not spawn an upgrade on despawn breakable
                possibly_spawn_upgrade(
                    &mut commands,
                    &asset_server,
                    breakable_transform.translation,
                );
            }
            CollisionEvent::Stopped(_e1, _e2, _flags) => {
                // Collision OUT
            }
        }
    }
}

/// If the player and the flag collide, despawn the flag
///
/// Arguments:
///
/// * `collision_events`: EventReader<CollisionEvent>
/// * `player_query`: Query<(Entity, &mut Player), With<Player>>
/// * `flag_query`: Query<(Entity, &mut Flag), Without<Player>>,
/// * `commands`: Commands
pub fn player_and_flag_collision_listener(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut Player), With<Player>>,
    mut flag_query: Query<(Entity, &mut Flag), Without<Player>>,
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    mut audio: ResMut<DynamicAudioChannels>,
) {
    //Iterate over collision events
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_1, entity_2, _flags) => {
                if flag_query.is_empty() {
                    break;
                }

                //If found an event, check if envolves the player
                let (player_entity, _player) = player_query.single_mut();
                let (flag_entity, _flag) = flag_query.single_mut();

                let has_player_collide = player_entity == *entity_1 || player_entity == *entity_2;
                let has_flag_collide = flag_entity == *entity_1 || flag_entity == *entity_2;
                println!(
                    "Player and flag collided ? {:?}",
                    has_player_collide && has_flag_collide
                );
                //If event is not related to player or flag, ignore it, another
                //listener should handle it
                if !has_player_collide || !has_flag_collide {
                    break;
                }
                item_collision(
                    &mut commands,
                    flag_entity.to_owned(),
                    asset_server.to_owned(),
                    audio.create_channel(SFX_AUDIO_CHANNEL),
                    String::from("audios/sfx/won_level_1.ogg"),
                );
                game_state.set(GameState::NextLevel);
            }
            CollisionEvent::Stopped(_e1, _e2, _flags) => {
                // Collision OUT
            }
        }
    }
}

/// Despawn the item entity and play a sound effect.
///
/// Arguments:
///
/// * `commands`: &mut Commands,
/// * `item_entity`: The entity of the item that was collected
/// * `asset_server`: The asset server that we created in the previous section.
/// * `audio`: &DynamicAudioChannel - This is the audio channel that we created in the previous step.
/// * `audio_source`: The name of the audio file to play.
pub fn item_collision(
    commands: &mut Commands,
    item_entity: Entity,
    asset_server: AssetServer,
    audio: &DynamicAudioChannel,
    audio_source: String,
) {
    // Despawn item
    commands.entity(item_entity).despawn_recursive();
    //Play Sound Effect
    play_sfx(audio, asset_server, audio_source)
}
