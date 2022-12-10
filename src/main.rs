use audio::GameAudioPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use item::ItemPlugin;
use map::MapPlugin;
use player::PlayerPlugin;
use simula_action::ActionPlugin;
use simula_camera::{flycam::*, orbitcam::*};
use state::GameState;

pub mod audio;
pub mod constants;
pub mod item;
pub mod map;
pub mod player;
pub mod state;
pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

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
        .add_state(GameState::Menu)
        .add_plugin(GameAudioPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(ActionPlugin)
        .add_plugin(OrbitCameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ItemPlugin)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 11.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.5),
        ..default()
    });
}
