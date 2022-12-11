use bevy::prelude::{
    App, AssetServer, Commands, Component, DespawnRecursiveExt, Entity, EventReader, Plugin, Query,
    Res, ResMut, SystemSet, With, Without,
};
use bevy_kira_audio::{DynamicAudioChannel, DynamicAudioChannels};
use bevy_rapier3d::prelude::CollisionEvent;

use crate::{
    audio::play_sfx, bomb::Bomb, constants::SFX_AUDIO_CHANNEL, map::Breakable, player::Player,
    utils::animate_interactive_items, GameState,
};

#[derive(Component)]
pub struct InteractiveItem;
pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(player_collision_listener)
                .with_system(animate_interactive_items)
                .with_system(explosion_collision_listener),
        );
    }
}

pub fn player_collision_listener(
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
                //TODO: Custom upgrades
                player.speed += 0.1;
            }
            CollisionEvent::Stopped(_e1, _e2, _flags) => {
                // Collision OUT
            }
        }
    }
}

pub fn explosion_collision_listener(
    mut collision_events: EventReader<CollisionEvent>,
    bomb_query: Query<Entity, With<Bomb>>,
    breakable_query: Query<(Entity, &Breakable), Without<Bomb>>,
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

                // Despawn breakable and play explosion sound
                item_collision(
                    &mut commands,
                    breakable_entity.to_owned(),
                    asset_server.to_owned(),
                    audio.create_channel(SFX_AUDIO_CHANNEL),
                    String::from("audios/sfx/bomb_explosion.ogg"),
                )
            }
            CollisionEvent::Stopped(_e1, _e2, _flags) => {
                // Collision OUT
            }
        }
    }
}
/// "When an item collides with the player, despawn the item, play a sound, and give the player an
/// upgrade."
///
/// The first thing we do is despawn the item. We do this by getting the `commands` resource and calling
/// `.entity(item_entity)` to get the `EntityCommandBuffer` for the item. Then we call
/// `.despawn_recursive()` to despawn the item and all of its children
///
/// Arguments:
///
/// * `commands`: &mut Commands,
/// * `item_entity`: The entity of the item that was picked up
/// * `asset_server`: This is the asset server that we created in the previous section.
/// * `audio`: &DynamicAudioChannel - This is the audio channel that we created in the previous step.
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
