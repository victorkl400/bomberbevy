use bevy::{ecs::component::ComponentStorage, prelude::*};
use bevy_kira_audio::{
    Audio, AudioApp, AudioChannel, AudioControl, AudioPlugin, AudioSource, DynamicAudioChannel,
    DynamicAudioChannels,
};

#[derive(Resource, Component, Default, Clone)]
struct BackgroundChannel;
#[derive(Resource, Component, Default, Clone)]
pub struct SoundEffectChannel;

#[derive(Resource)]
pub struct AudioState {
    background_handle: Handle<AudioSource>,
    volume: f64,
}
pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_audio_channel::<BackgroundChannel>()
            .add_audio_channel::<SoundEffectChannel>()
            .add_startup_system(start_bg_music::<BackgroundChannel>)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_audio);
    }
}

fn load_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_handle: Handle<AudioSource> = asset_server.load("audios/background/level1.ogg");

    commands.insert_resource(AudioState {
        background_handle,
        volume: 0.1,
    });
}

pub fn play_sfx(channel: &DynamicAudioChannel, asset_server: AssetServer) {
    let sfx_item_handle: Handle<AudioSource> = asset_server.load("audios/sfx/get_item.ogg");
    channel.set_volume(0.15);
    channel.play(sfx_item_handle);
}

fn start_bg_music<T: Component + Default>(
    channel: Res<AudioChannel<T>>,
    audio_handles: Res<AudioState>,
) {
    channel.set_volume(audio_handles.volume);
    channel.play(audio_handles.background_handle.clone());
}
