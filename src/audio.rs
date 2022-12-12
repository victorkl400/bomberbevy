use bevy::prelude::*;
use bevy_kira_audio::{
    AudioApp, AudioChannel, AudioControl, AudioPlugin, AudioSource, DynamicAudioChannel,
};

use crate::GameState;

#[derive(Resource, Component, Default, Clone)]
struct BackgroundChannel;
#[derive(Resource, Component, Default, Clone)]
pub struct SoundEffectChannel;

#[derive(Resource)]
pub struct AudioState {
    level_1: Handle<AudioSource>,
    game_over_1: Handle<AudioSource>,
    game_over_2: Handle<AudioSource>,
    game_over_3: Handle<AudioSource>,
    volume: f64,
}
pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_audio_channel::<BackgroundChannel>()
            .add_audio_channel::<SoundEffectChannel>()
            .add_system_set(
                SystemSet::on_enter(GameState::Gameplay)
                    .with_system(start_bg_music::<BackgroundChannel>),
            )
            .add_startup_system_to_stage(StartupStage::PreStartup, load_audio)
            .add_system_set(
                SystemSet::on_exit(GameState::Gameplay)
                    .with_system(stop_bg_music::<BackgroundChannel>),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::GameOver)
                    .with_system(play_game_over::<BackgroundChannel>),
            );
    }
}

/// `load_audio` loads the audio file `level1.ogg` from the `audios/background` folder and stores it in
/// the `AudioState` resource
///
/// Arguments:
///
/// * `commands`: Commands - This is the command buffer that we will use to insert the resource into the
/// world.
/// * `asset_server`: Res<AssetServer> - This is the asset server that we'll use to load the audio file.
fn load_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level_1: Handle<AudioSource> = asset_server.load("audios/background/level1.ogg");
    let game_over_1: Handle<AudioSource> = asset_server.load("audios/sfx/game_over_1.ogg");
    let game_over_2: Handle<AudioSource> = asset_server.load("audios/sfx/game_over_2.ogg");
    let game_over_3: Handle<AudioSource> = asset_server.load("audios/sfx/game_over_3.ogg");

    commands.insert_resource(AudioState {
        level_1,
        game_over_1,
        game_over_2,
        game_over_3,
        volume: 0.1,
    });
}

/// It plays a sound effect
///
/// Arguments:
///
/// * `channel`: The channel to play the sound on.
/// * `asset_server`: AssetServer - This is the asset server that we created in the previous section.
pub fn play_sfx(channel: &DynamicAudioChannel, asset_server: AssetServer, path: String) {
    let sfx_item_handle: Handle<AudioSource> = asset_server.load(path);
    channel.set_volume(0.15);
    channel.play(sfx_item_handle);
}

/// "If the `AudioChannel` is not playing, start playing the background music."
///
/// The first line of the function is a function signature. It says that the function takes two
/// arguments, a `Res<AudioChannel<T>>` and a `Res<AudioState>`. The `Res` is a resource, which is a
/// type of data that is shared across the entire game. The `AudioChannel` is a resource that is used to
/// play audio. The `AudioState` is a resource that contains the audio handles for the background music
/// and the sound effects
///
/// Arguments:
///
/// * `channel`: The audio channel to play the music on.
/// * `audio_handles`: This is the resource that contains the audio handles.
fn start_bg_music<T: Component + Default>(
    channel: Res<AudioChannel<T>>,
    audio_handles: Res<AudioState>,
    game_state: Res<State<GameState>>,
) {
    channel.play(audio_handles.level_1.clone());
    channel.set_volume(audio_handles.volume);
}

fn play_game_over<T: Component + Default>(
    channel: Res<AudioChannel<T>>,
    audio_handles: Res<AudioState>,
) {
    channel.play(audio_handles.game_over_1.clone());
}

fn stop_bg_music<T: Component + Default>(
    channel: Res<AudioChannel<T>>,
    audio_handles: Res<AudioState>,
) {
    channel.stop();
}
