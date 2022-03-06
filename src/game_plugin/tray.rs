use crate::game_plugin::SystemLabels;
use crate::{game_plugin::actions::KeyActions, TRAY_SIZE};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub struct TrayPlugin;

#[derive(Component)]
struct Tray;

impl Plugin for TrayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup_tray)
            .add_system(
                tray_update
                    .before(SystemLabels::EvaluateInput)
                    .after(SystemLabels::GatherInput),
            );
    }
}

fn setup_tray(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(5.0),
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    top: Val::Px(0.0),
                },
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                alignment: TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Left,
                },
                sections: vec![
                    TextSection {
                        // Non-potential word letters
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/OverpassMono-Bold.ttf"),
                            font_size: 120.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        // Potential Word
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/OverpassMono-Bold.ttf"),
                            font_size: 120.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        // Underlines
                        value: "__________".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/OverpassMono-Bold.ttf"),
                            font_size: 120.0,
                            color: Color::GRAY,
                        },
                    },
                    TextSection {
                        // Overflow section
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/OverpassMono-Bold.ttf"),
                            font_size: 120.0,
                            color: Color::RED,
                        },
                    },
                ],
            },
            ..Default::default()
        })
        .insert(Tray);
}

fn tray_update(key_actions: ResMut<KeyActions>, mut query: Query<&mut Text, With<Tray>>) {
    for mut text in query.iter_mut() {
        let stack = key_actions.char_stack.clone();
        let mut split_index = if let Some(word) = &key_actions.longest_word_option {
            stack.len() - word.len()
        } else {
            stack.len()
        };

        // Don't show any as yellow if we are overflowed
        if stack.len() > TRAY_SIZE {
            split_index = TRAY_SIZE;
        }

        let stack_length = stack.len().min(TRAY_SIZE);

        text.sections[0].value = stack[0..split_index]
            .iter()
            .collect::<String>()
            .to_string()
            .to_uppercase();
        text.sections[1].value = stack[split_index..stack_length]
            .iter()
            .collect::<String>()
            .to_string()
            .to_uppercase();
        text.sections[2].value = "_".repeat(TRAY_SIZE - stack_length);

        if stack.len() > TRAY_SIZE {
            text.sections[3].value = stack[TRAY_SIZE..]
                .iter()
                .collect::<String>()
                .to_string()
                .to_uppercase();
        }
    }
}
