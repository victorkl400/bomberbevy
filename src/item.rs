use bevy::prelude::{
    App, AssetServer, Commands, DespawnRecursiveExt, Entity, EventReader, Plugin, Query, Res,
    ResMut, With,
};
use bevy_kira_audio::{DynamicAudioChannel, DynamicAudioChannels};
use bevy_rapier3d::prelude::CollisionEvent;

use crate::{audio::play_sfx, player::Player};

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collide_listener);
    }
}
/// "When a collision event occurs, check if the player is involved, and if so, despawn the item and
/// play a sound."
///
/// The first thing we do is create a `Query` for the player. This is a way to get a reference to the
/// player entity. We use the `single_mut` method to get a mutable reference to the player entity
///
/// Arguments:
///
/// * `collision_events`: EventReader<CollisionEvent>
/// * `player_query`: Query<Entity, With<Player>>
/// * `commands`: Commands - This is a resource that allows you to add, remove, and modify entities.
/// * `asset_server`: Res<AssetServer> - This is a resource that allows us to load assets.
/// * `audio`: ResMut<DynamicAudioChannels>
pub fn collide_listener(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<Entity, With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut audio: ResMut<DynamicAudioChannels>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_1, entity_2, _flags) => {
                let player_entity = player_query.single_mut();
                let item_entity = if player_entity == *entity_1 {
                    entity_2
                } else {
                    entity_1
                };
                // Collision IN
                println!("Player has collided with item {:?} ", item_entity);
                // Despawn item
                item_collision(
                    &mut commands,
                    item_entity.to_owned(),
                    asset_server.to_owned(),
                    audio.create_channel("sfx"),
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
) {
    // Despawn item
    commands.entity(item_entity).despawn_recursive();
    //Play Sound

    play_sfx(audio, asset_server)
    //Give Player Upgrade
}
