use crate::game_plugin::actions::KeyActions;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct FPSCounterPlugin;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct ColorText;

impl Plugin for FPSCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup)
            .add_system(text_update_system)
            .add_system(text_color_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default()); // TODO: Should this be somewhere else?

    // Text with one section
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(25.),
                    right: Val::Px(25.),
                    bottom: Val::Px(25.),
                    top: Val::Px(420.),
                },
                max_size: Size::new(Val::Px(910.), Val::Px(515.)),
                ..Default::default()
            },
            text: Text::with_section(
                "This is example text at the bottom of the screen. Two whole complete sentences worth! Wowzers! This is example text at the bottom of the screen. Two whole complete sentences worth! Wowzers! This is example text at the bottom of the screen. Two whole complete sentences worth! Wowzers!",
                TextStyle {
                    font: asset_server.load("fonts/ShareTechMono-Regular.ttf"),
                    font_size: 25.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(ColorText);

    // Rich text with multiple sections
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 120.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 120.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);
}

fn text_update_system(
    diagnostics: Res<Diagnostics>,
    key_actions: ResMut<KeyActions>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in query.iter_mut() {
        let stack = key_actions.char_stack.clone();
        let split_index = if let Some(word) = &key_actions.longest_word_option {
            stack.len() - word.len() // TODO: if this line errors, it's probably because the system was run in the wrong order. We need labels...
        } else {
            stack.len()
        };

        //if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        //if let Some(average) = fps.average() {
        // Update the value of the second section
        text.sections[0].value = format!("{}", stack[0..split_index].iter().collect::<String>());
        text.sections[1].value = format!("{}", stack[split_index..].iter().collect::<String>());
        //}
        //}
    }
}

fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    for mut text in query.iter_mut() {
        let seconds = time.seconds_since_startup() as f32;
        // We used the `Text::with_section` helper method, but it is still just a `Text`,
        // so to update it, we are still updating the one and only section
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}
