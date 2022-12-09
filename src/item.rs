use bevy::prelude::{App, Commands, DespawnRecursiveExt, Entity, EventReader, Plugin};
use bevy_rapier3d::prelude::CollisionEvent;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collide_listener);
    }
}
pub fn collide_listener(mut collision_events: EventReader<CollisionEvent>, mut commands: Commands) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(item_entity, _e2, _flags) => {
                // Collision IN
                println!("Player has collided with item {:?}", item_entity);
                // Despawn item
                item_collision(&mut commands, item_entity.to_owned())
            }
            CollisionEvent::Stopped(_e1, _e2, _flags) => {
                // Collision OUT
            }
        }
    }
}
pub fn item_collision(commands: &mut Commands, item_entity: Entity) {
    // Despawn item
    commands.entity(item_entity).despawn_recursive()
    //Play Sound
    //Give Player Upgrade
}
