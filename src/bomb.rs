use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::DynamicAudioChannels;
use bevy_rapier3d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, Sensor};

use crate::{
    audio::play_sfx,
    constants::{BOMB_EXPLOSTION_TIME, BOMB_SPAWN_DELAY, SFX_AUDIO_CHANNEL},
    map::Breakable,
    player::Player,
    GameState,
};

pub struct BombPlugin;

#[derive(Component)]
pub struct Bomb {
    explode_timer: Timer,
}
impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(drop_bomb)
                .with_system(explode_bomb),
        );
    }
}

/// "If the space bar is pressed, spawn a bomb at the player's position."
///
/// The first thing we do is get a mutable reference to the player's transform. We do this by creating a
/// `Query` that looks for entities with the `Player` component and a mutable `Transform` component. We
/// then use the `single_mut` method to get a mutable reference to the first entity that matches the
/// query
///
/// Arguments:
///
/// * `commands`: Commands - This is the resource that allows us to spawn entities.
/// * `meshes`: ResMut<Assets<Mesh>>,
/// * `materials`: ResMut<Assets<StandardMaterial>>,
/// * `player_query`: Query<(&Player, &mut Transform)>
/// * `keyboard`: Res<Input<KeyCode>>
fn drop_bomb(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut audio: ResMut<DynamicAudioChannels>,
) {
    let (mut player, player_transform) = player_query.single_mut();
    let player_pos = player_transform.clone().translation;
    player.bomb_delay.tick(time.delta());

    let bomb_explosion_range = vec![
        (
            Vec3::ZERO,
            Quat::from_rotation_z(0.0),
            Collider::cuboid(player.bomb_range, 0.1, 0.1),
        ),
        (
            Vec3::ZERO,
            Quat::from_rotation_z(0.0),
            Collider::cuboid(0.1, 0.1, player.bomb_range),
        ),
    ];

    if player.bomb_delay.finished()
        && keyboard.just_pressed(KeyCode::Space)
        && player.bomb_amount > 0
    {
        commands
            .spawn(SceneBundle {
                scene: asset_server.load("objects/bomb.glb#Scene0"),
                transform: Transform {
                    translation: Vec3::new(player_pos.x, player_pos.y + 0.1, player_pos.z),
                    scale: Vec3::new(0.5, 0.5, 0.5),
                    ..Default::default()
                },
                ..default()
            })
            .insert(Name::new("Bomb"))
            .insert(Bomb {
                explode_timer: Timer::new(
                    Duration::from_secs(BOMB_EXPLOSTION_TIME),
                    TimerMode::Once,
                ),
            })
            .insert(Collider::compound(bomb_explosion_range))
            .insert(Sensor);
        player.bomb_delay = Timer::new(Duration::from_millis(BOMB_SPAWN_DELAY), TimerMode::Once);
        player.bomb_amount -= 1; //Decrease amount
        play_sfx(
            audio.create_channel(SFX_AUDIO_CHANNEL),
            asset_server.to_owned(),
            String::from("audios/sfx/bomb_start.ogg"),
        );
    }
}

fn explode_bomb(
    mut commands: Commands,
    mut bomb_query: Query<(Entity, &mut Bomb), Without<Breakable>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut audio: ResMut<DynamicAudioChannels>,
) {
    for (bomb_entity, mut bomb) in bomb_query.iter_mut() {
        // timers gotta be ticked, to work
        bomb.explode_timer.tick(time.delta());

        //Miliseconds before explode, add collider to despawn breakables
        if bomb.explode_timer.percent_left() <= 0.01 {
            commands
                .entity(bomb_entity)
                .insert(ActiveCollisionTypes::KINEMATIC_STATIC)
                .insert(ActiveEvents::COLLISION_EVENTS);
        };
        // if it finished, despawn the bomb
        if bomb.explode_timer.finished() {
            //Despawn bomb
            commands.entity(bomb_entity).despawn_recursive();

            //Play explosion sound
            play_sfx(
                audio.create_channel(SFX_AUDIO_CHANNEL),
                asset_server.to_owned(),
                String::from("audios/sfx/bomb_explosion.ogg"),
            );
        }
    }
}
