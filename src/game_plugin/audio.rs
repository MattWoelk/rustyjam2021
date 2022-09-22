use crate::game_plugin::loading::AudioAssets;
use crate::game_plugin::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioPlugin};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_audio));
    }
}

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.set_volume(0.3);
    audio.play(audio_assets.flying.clone()).looped();
    audio.pause();
}
