use bevy::prelude::*;
use bevy::utils::Duration;

pub struct SecondsTimerPlugin;
struct SecondsTimer;

impl Plugin for SecondsTimerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_timer.system())
            .add_system(update_timer.system());
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
                //max_size: Size::new(Val::Px(910.), Val::Px(50.)),
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
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
        .insert(Timer::from_seconds(9999., true))
        .insert(SecondsTimer);
}

fn update_timer(time: Res<Time>, mut query: Query<(&mut Text, &mut Timer, With<SecondsTimer>)>) {
    for (mut text, mut timer, _) in query.iter_mut() {
        timer.tick(time.delta());
        let total = timer.elapsed_secs() + timer.times_finished() as f32;
        text.sections[0].value = format!("{:.2} seconds", total);
    }
}
