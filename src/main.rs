// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod game_plugin;

//use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::WindowPlugin;
use bevy::DefaultPlugins;
use game_plugin::GamePlugin;

use bevy_prototype_lyon::prelude::*;

pub const SCREEN_WIDTH: f32 = 960.;
pub const SCREEN_HEIGHT: f32 = 540.;
pub const TRAY_SIZE: usize = 10;

fn main() {
    let mut app = App::new();
    app
        // .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                title: "Bevy game".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        //.add_plugins_with(DefaultPlugins, |plugins| plugins.disable::<LogPlugin>())
        .add_plugin(ShapePlugin)
        .add_plugin(GamePlugin);

    //bevy_mod_debugdump::print_schedule(&mut app);
    //bevy_mod_debugdump::print_render_graph(&mut app);

    app.run();
}
