mod actions;
mod audio;
mod enemy;
mod loading;
mod menu;
mod player;
mod player_text_input;
mod seconds_timer;

use crate::game_plugin::actions::ActionsPlugin;
use crate::game_plugin::audio::InternalAudioPlugin;
use crate::game_plugin::enemy::EnemyPlugin;
use crate::game_plugin::loading::LoadingPlugin;
use crate::game_plugin::menu::MenuPlugin;
use crate::game_plugin::player::PlayerPlugin;
use crate::game_plugin::player_text_input::PlayerTextInputPlugin;
use crate::game_plugin::seconds_timer::SecondsTimerPlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum SystemLabels {
    GatherInput,
    EvaluateInput,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerTextInputPlugin)
            .add_plugin(SecondsTimerPlugin)
            //.add_plugin(WorldInspectorPlugin::new())
            .add_system(bevy::input::system::exit_on_esc_system);
    }
}
