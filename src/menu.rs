use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::{app::AppExit, prelude::*};
use bevy_kira_audio::DynamicAudioChannels;

use crate::audio::play_sfx;
use crate::constants::SFX_AUDIO_CHANNEL;
use crate::{
    constants::{HEIGHT, WIDTH},
    GameState,
};

#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub struct CloseButton;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(spawn_main_menu))
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(any_button_pressed)
                    .with_system(close_button_clicked),
            );
    }
}

/// When any button is pressed, despawn the menu UI and set the game state to `Gameplay`
///
/// Arguments:
///
/// * `commands`: Commands is a struct that allows you to create, delete, and modify entities.
/// * `key_evr`: EventReader<KeyboardInput>
/// * `menu_root`: Query<Entity, With<MenuUI>>
/// * `game_state`: The game state resource.
fn any_button_pressed(
    mut commands: Commands,
    mut key_evr: EventReader<KeyboardInput>,
    menu_root: Query<Entity, With<MenuUI>>,
    mut game_state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    mut audio: ResMut<DynamicAudioChannels>,
) {
    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                let root_entity = menu_root.single();
                commands.entity(root_entity).despawn_recursive();
                play_sfx(
                    audio.create_channel(SFX_AUDIO_CHANNEL),
                    asset_server.to_owned(),
                    String::from("audios/sfx/menu_click.ogg"),
                );

                game_state.set(GameState::Gameplay).unwrap();
            }
            ButtonState::Released => {}
        }
    }
}

/// "If the close button is clicked, send an `AppExit` event."
///
/// The `close_button_clicked` function is a system. It's a function that runs every frame. It's a
/// function that takes two arguments:
///
/// - `interactions`: A query that returns all the `Interaction` components that are attached to
/// `CloseButton` entities.
/// - `mut exit`: An event writer that allows us to send `AppExit` events
///
/// Arguments:
///
/// * `interactions`: Query<&Interaction, (With<CloseButton>, Changed<Interaction>)>
/// * `exit`: EventWriter<AppExit>
fn close_button_clicked(
    interactions: Query<&Interaction, (With<CloseButton>, Changed<Interaction>)>,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            exit.send(AppExit);
        }
    }
}

/// We spawn a button and an image, and then we add the button as a child of the image
///
/// Arguments:
///
/// * `commands`: Commands - This is the commands object that is used to spawn entities.
/// * `asset_server`: Res<AssetServer>
fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Spawn a close button
    let close_button = commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(40.0), Val::Px(40.0)),
                align_self: AlignSelf::FlexStart,
                justify_content: JustifyContent::FlexStart,
                margin: UiRect::all(Val::Percent(2.0)),
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        })
        .insert(CloseButton)
        .id();

    //Spawn Menu Background image
    commands
        .spawn(ImageBundle {
            style: Style {
                size: Size::new(Val::Px(WIDTH), Val::Px(HEIGHT)),
                ..default()
            },
            image: asset_server.load("images/menu_bomberbevy.png").into(),
            ..default()
        })
        .add_child(close_button)
        .insert(MenuUI);
}
