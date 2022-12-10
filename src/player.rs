use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::{
        Collider, ExternalForce, KinematicCharacterController, NoUserData, RapierPhysicsPlugin,
        Restitution, RigidBody,
    },
    render::RapierDebugRenderPlugin,
};

use crate::map::Breakable;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    speed: f32,
    bomb_delay: Timer,
}
#[derive(Component)]
pub struct Bomb {
    range: f32,
    explode_timer: Timer,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(drop_bomb)
            .add_system(explode_bomb);
        // .add_system(rotate_system);
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
                scale: Vec3::new(0.57, 1., 0.57),
                ..Default::default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.25, 0.1, 0.4))
        .insert(ExternalForce {
            force: Vec3::ZERO,
            torque: Vec3::ZERO,
        })
        .insert(KinematicCharacterController { ..default() })
        .insert(Restitution::coefficient(0.1))
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 1.0,
            bomb_delay: Timer::new(Duration::from_millis(250), TimerMode::Once),
        });
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut player_transform) = player_query.single_mut();
    let player_pos = player_transform.clone().translation;
    player.bomb_delay.tick(time.delta());
    if player.bomb_delay.finished() && keyboard.just_released(KeyCode::Space) {
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(
                    Mesh::try_from(shape::Icosphere {
                        radius: 0.2,
                        subdivisions: 32,
                    })
                    .unwrap(),
                ),
                material: materials.add(StandardMaterial {
                    base_color: Color::hex("000000").unwrap(),
                    ..default()
                }),
                transform: Transform {
                    translation: Vec3::new(player_pos.x, player_pos.y + 0.1, player_pos.z),
                    ..default()
                },
                ..default()
            })
            .insert(Name::new("Bomb"))
            .insert(Bomb {
                range: 2.,
                explode_timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
            });
        player.bomb_delay = Timer::new(Duration::from_millis(250), TimerMode::Once);
    }
}

fn explode_bomb(
    mut commands: Commands,
    // mut breakables: Query<(Entity, &mut Breakable, &mut Transform)>,
    mut bomb_query: Query<(Entity, &mut Bomb, &mut Transform)>,
    time: Res<Time>,
) {
    for (bomb_entity, mut bomb, bomb_transform) in bomb_query.iter_mut() {
        // timers gotta be ticked, to work
        bomb.explode_timer.tick(time.delta());

        // if it finished, despawn the bomb
        if bomb.explode_timer.finished() {
            // for (breakable_entity, mut breakable, breakable_transform) in breakables.iter_mut() {
            // breakable_transform.translation.x <= bomb_transform.translation.x + bomb.range

            //Despawn bomb
            commands.entity(bomb_entity).despawn_recursive();
            // }
        }
    }
}
