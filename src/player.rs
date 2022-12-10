use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::{
        Collider, ExternalForce, KinematicCharacterController, NoUserData, RapierPhysicsPlugin,
        Restitution, RigidBody,
    },
    render::RapierDebugRenderPlugin,
};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub bomb_delay: Timer,
    pub bomb_range: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

/// "If the player is pressing W, move the player forward. If the player is pressing S, move the player
/// backward. If the player is pressing A, move the player left. If the player is pressing D, move the
/// player right."
///
/// The first thing we do is get the player and transform components from the player_query. We need the
/// player component to get the player's speed, and we need the transform component to move the player
///
/// Arguments:
///
/// * `controllers`: Query<&mut KinematicCharacterController>
/// * `player_query`: Query<(&Player, &mut Transform)>
/// * `keyboard`: Res<Input<KeyCode>>
/// * `time`: Res<Time> - This is the time resource. It's a resource because it's a global value that
/// can be accessed from anywhere.
fn player_movement(
    mut controllers: Query<&mut KinematicCharacterController>,
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();

    let mut z_delta = 0.0;
    if keyboard.pressed(KeyCode::W) {
        z_delta -= player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        z_delta += player.speed * time.delta_seconds();
    }

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::A) {
        x_delta -= player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        x_delta += player.speed * time.delta_seconds();
    }

    let target = transform.translation + Vec3::new(x_delta, 0.0, z_delta);
    transform.translation = target;

    for mut controller in controllers.iter_mut() {
        controller.translation = Some(Vec3::new(x_delta, 0.0, z_delta));
    }
}

/// We spawn a cube, give it a rigid body, a collider, a name, and a player component
///
/// Arguments:
///
/// * `commands`: Commands - This is the command buffer that we will use to spawn the player.
/// * `asset_server`: Res<AssetServer>
fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // cube
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("objects/enemy_ufoRedWeapon.glb#Scene0"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.4, 0.2),
                scale: Vec3::new(0.5, 0.8, 0.5),
                ..Default::default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.4, 0.2, 0.4))
        .insert(ExternalForce {
            force: Vec3::ZERO,
            torque: Vec3::ZERO,
        })
        .insert(KinematicCharacterController { ..default() })
        .insert(Restitution::coefficient(0.1))
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 1.0,
            bomb_delay: Timer::new(Duration::from_millis(350), TimerMode::Once),
            bomb_range: 1.0,
        });
}
