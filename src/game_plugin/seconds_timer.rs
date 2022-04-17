use crate::game_plugin::GameState;
use bevy::prelude::*;

pub struct SecondsTimerPlugin;

#[derive(Component, Deref, DerefMut)]
struct SecondsTimer(Timer);

impl Plugin for SecondsTimerPlugin {
    fn build(&self, app: &mut App) {
        // TODO: make the timer only appear when a match starts
        app.add_startup_system(spawn_timer).add_system(update_timer);
    }
}

fn spawn_timer(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_content: AlignContent::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(425.),
                    right: Val::Px(25.),
                    bottom: Val::Px(540. - 50.),
                    top: Val::Px(25.),
                },
                ..Default::default()
            },
            text: Text::with_section(
                "123.12s",
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
        .insert(SecondsTimer(Timer::from_seconds(9999., true)));
}

fn update_timer(
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut SecondsTimer)>,
    state: ResMut<State<GameState>>,
) {
    if state.current() == &GameState::Playing {
        for (mut text, mut timer) in query.iter_mut() {
            timer.tick(time.delta());
            let total = timer.elapsed_secs() + timer.times_finished() as f32;
            text.sections[0].value = format!("{total:.2} seconds");
        }
    }
}
