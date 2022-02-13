mod actions;
mod audio;
mod enemy;
mod loading;
mod menu;
mod player;
mod seconds_timer;
mod tray;

use crate::game_plugin::actions::ActionsPlugin;
use crate::game_plugin::audio::InternalAudioPlugin;
use crate::game_plugin::enemy::EnemyPlugin;
use crate::game_plugin::loading::LoadingPlugin;
use crate::game_plugin::menu::MenuPlugin;
use crate::game_plugin::player::PlayerPlugin;
use crate::game_plugin::seconds_timer::SecondsTimerPlugin;
use crate::game_plugin::tray::TrayPlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::prelude::*;
//use bevy_inspector_egui::WorldInspectorPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    /// During the loading State the LoadingPlugin will load our assets
    Loading,

    /// During this State the actual game logic is executed
    Playing,

    /// Boss Fight! (overlay this on top of "Playing", using push and pop)
    Boss,

    /// A lose condition has been hit (overlay this on top of "Playing", using push and pop)
    PlayingLose, // TODO: this should be part of PlayState, if that works well.

    /// Here the menu is drawn and waiting for player interaction
    Menu,
}

#[allow(unused)]
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum PlayState {
    /// Normal play
    Running,
    /// when the boss battle stats
    BossBattle,
    /// juicy pauses when significant things happen (push and pop this)
    HitPaused,
    /// When a round is over
    Finished,
}

#[derive(Clone, PartialEq, Debug)]
struct PlayInfo {
    state: PlayState,
    hit_pause_timer: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum SystemLabels {
    GatherInput,
    EvaluateInput,
    MoveEnemies,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            //.add_state(PlayState::Running)
            .add_stage_after(CoreStage::Update, "resolve", SystemStage::single_threaded())
            .insert_resource(PlayInfo {
                state: PlayState::Running,
                hit_pause_timer: 0.,
            })
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(TrayPlugin)
            .add_plugin(SecondsTimerPlugin)
            //.add_plugin(WorldInspectorPlugin::new())
            .add_system(bevy::input::system::exit_on_esc_system);
    }
}
