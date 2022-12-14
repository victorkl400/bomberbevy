use bevy::prelude::*;
use bevy_kira_audio::DynamicAudioChannels;

use crate::{
    audio::play_sfx,
    constants::{HEIGHT, SFX_AUDIO_CHANNEL, WIDTH},
    GameState, Level,
};

#[derive(Component)]
pub struct NextLevelUI;

#[derive(Component)]
pub struct CloseButton;

pub struct NextLevelPlugin;

impl Plugin for NextLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::NextLevel).with_system(spawn_next_level_screen),
        )
        .add_system_set(SystemSet::on_update(GameState::NextLevel).with_system(enter_pressed));
    }
}

fn enter_pressed(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    next_level_root: Query<Entity, With<NextLevelUI>>,
    mut game_state: ResMut<State<GameState>>,
    mut level_state: ResMut<State<Level>>,
    asset_server: Res<AssetServer>,
    mut audio: ResMut<DynamicAudioChannels>,
) {
    if keyboard.just_pressed(KeyCode::Return) {
        let root_entity = next_level_root.single();
        commands.entity(root_entity).despawn_recursive();

        //TODO: Improve this level update
        if level_state.current().to_owned() == Level::Level1 {
            level_state.set(Level::Level2).unwrap();
        }
        if level_state.current().to_owned() == Level::Level2 {
            level_state.set(Level::Level3).unwrap();
        }

        if level_state.current().to_owned() != Level::Level3 {
            game_state.set(GameState::Gameplay).unwrap();
        } else {
            level_state.set(Level::Level1).unwrap();
            game_state.set(GameState::Menu).unwrap();
        }
        play_sfx(
            audio.create_channel(SFX_AUDIO_CHANNEL),
            asset_server.to_owned(),
            String::from("audios/sfx/menu_click.ogg"),
        );
    }
}
/// We create a TextBundle with a Text that has a single section, and we set the alignment of the Text
/// to be centered. We then set the style of the TextBundle itself to be positioned at the bottom right
/// of the screen
///
/// Arguments:
///
/// * `commands`: Commands - This is the main way to spawn entities into the world.
/// * `asset_server`: Res<AssetServer>
fn spawn_next_level_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Spawn Menu Background image
    commands
        .spawn(ImageBundle {
            style: Style {
                size: Size::new(Val::Px(WIDTH), Val::Px(HEIGHT)),
                ..default()
            },
            image: asset_server.load("images/next_level_bomberbevy.png").into(),
            ..default()
        })
        .insert(NextLevelUI);
}
