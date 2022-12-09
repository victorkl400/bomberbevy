use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier3d::{
    prelude::{
        Collider, ExternalForce, KinematicCharacterControllerOutput, NoUserData,
        RapierPhysicsPlugin, Restitution, RigidBody,
    },
    render::RapierDebugRenderPlugin,
};

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(modify_character_controller_slopes);
    }
}

fn player_movement(
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
}
fn modify_character_controller_slopes(
    mut character_controller_outputs: Query<&mut KinematicCharacterControllerOutput>,
) {
    for mut output in character_controller_outputs.iter_mut() {
        for collision in &output.collisions {
            // Do something with that collision information.
            println!("colisao {:?}", collision);
        }
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // cube
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("objects/enemy_ufoRed.glb#Scene0"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.4, 0.0),
                scale: Vec3::new(0.57, 1., 0.57),
                ..Default::default()
            },
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.3, 0.3, 0.3))
        .insert(ExternalForce {
            force: Vec3::ZERO,
            torque: Vec3::ZERO,
        })
        .insert(Restitution::coefficient(0.1))
        .insert(Player { speed: 1.4 });
}