use audio::GameAudioPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bomb::BombPlugin;
use collider::ColliderPlugin;
use constants::{HEIGHT, WIDTH};
use map::MapPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;
use simula_action::ActionPlugin;
use simula_camera::{flycam::*, orbitcam::*};

pub mod audio;
pub mod bomb;
pub mod collider;
pub mod constants;
pub mod map;
pub mod menu;
pub mod player;
pub mod utils;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Menu,
    Loading,
    Gameplay,
}

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
        .add_plugin(MenuPlugin)
        //External Mod Import
        .add_plugin(EguiPlugin)
        .add_plugin(ActionPlugin)
        .add_plugin(OrbitCameraPlugin)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        //Systems
        // .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
        //TODO despawn scene on returning to main menu (on_exit)
        .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(setup_basic_scene))
        .add_startup_system(spawn_camera)
        .run();
}

fn setup_basic_scene(mut commands: Commands) {
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

fn spawn_camera(mut commands: Commands) {
    // Spawn Camera
    let camera_location = Transform::from_xyz(0.0, 11.0, 7.0);
    commands.spawn(Camera3dBundle {
        transform: camera_location.looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
