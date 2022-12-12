use bevy::prelude::*;
use bevy_rapier3d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, RigidBody, Sensor};

use crate::{
    bomb::Bomb,
    constants::DEFAULT_OBJECT_SCALE,
    map::{Breakable, CustomProps, ObjectProps},
    player::Player,
    utils::{spawn_custom, spawn_floor},
    GameState,
};

pub struct GameLogicPlugin;

#[derive(Component)]
pub struct Flag;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(has_finalized)
                .with_system(has_lose),
        );
    }
}
fn has_finalized(
    mut commands: Commands,
    breakable_query: Query<(Entity, &Breakable, &Transform), Without<Flag>>,
    flag_query: Query<(Entity, &Flag, &Transform), Without<Breakable>>,
    asset_server: Res<AssetServer>,
) {
    if breakable_query.is_empty() && flag_query.is_empty() {
        println!("CONGRATULATIONS, YOU WON");
        let object_props = ObjectProps {
            add_floor: false,
            is_floor: true,
            interactive: false,
            path: "objects/flag.glb#Scene0".to_owned(),
            custom: Some(CustomProps {
                scale: DEFAULT_OBJECT_SCALE.to_owned(),
                rotation: Quat::from_rotation_y(45.0),
                sum_translation: Vec3::ZERO,
            }),
            breakable: false,
            name: String::from("FloorTeleport"),
        };
        let flag = spawn_custom(
            &mut commands,
            &object_props,
            &asset_server,
            Vec3::new(0.1, 0.2, -0.1),
        );
        commands
            .entity(flag)
            .insert(Flag)
            .insert(Sensor)
            .insert(ActiveCollisionTypes::KINEMATIC_STATIC)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Collider::cuboid(0.3, 0.3, 0.3))
            .insert(RigidBody::Fixed);
    }
}

fn has_lose(
    breakable_query: Query<(Entity, &Breakable), Without<Player>>,
    bomb_query: Query<(Entity, &Bomb), Without<Player>>,
    mut player_query: Query<(Entity, &Player), With<Player>>,
    mut game_state: ResMut<State<GameState>>,
) {
    let (player_ent, player) = player_query.single_mut();

    if !breakable_query.is_empty() && player.bomb_amount == 0 && bomb_query.is_empty() {
        println!("YOU LOSE");
        game_state.set(GameState::GameOver);
    }
}
