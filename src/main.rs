use audio::GameAudioPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bomb::BombPlugin;
use collider::ColliderPlugin;
use constants::{HEIGHT, WIDTH};
use map::MapPlugin;
use player::PlayerPlugin;
use simula_action::ActionPlugin;
use simula_camera::{flycam::*, orbitcam::*};
use state::GameState;

pub mod audio;
pub mod bomb;
pub mod collider;
pub mod constants;
pub mod map;
pub mod player;
pub mod state;
pub mod utils;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(255., 255., 255.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "BomberBevy".to_string(),
                resizable: false,
                ..default()
            },
            ..default()
        }))
        //Game State
        .add_state(GameState::Menu)
        //Custom Mod Import
        .add_plugin(MapPlugin)
        .add_plugin(GameAudioPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(BombPlugin)
        .add_plugin(ColliderPlugin)
        //External Mod Import
        .add_plugin(EguiPlugin)
        .add_plugin(ActionPlugin)
        .add_plugin(OrbitCameraPlugin)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        //Systems
        .add_startup_system(setup_basic_scene)
        .run();
}

fn setup_basic_scene(mut commands: Commands) {
    // Spawn Camera
    let camera_location = Transform::from_xyz(0.0, 11.0, 7.0);
    commands.spawn(Camera3dBundle {
        transform: camera_location.looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn Light
    let light_location = Transform::from_xyz(0.0, 5.0, 0.5);
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: light_location,
        ..default()
    });
}
