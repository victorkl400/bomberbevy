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
