use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use super::{enemy::Enemy, GameState};
pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Boss)
                .with_system(destroy_all_enemies)
                .with_system(spawn_boss),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Boss)
                .with_system(boss_letter_swarm)
                .with_system(boss_movement),
        );
    }
}

#[derive(Component)]
pub struct Boss {
    velocity: Vec3,
}

#[derive(Component)]
struct BossLetter {
    velocity: Vec3,
}

fn spawn_boss(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert(Boss {
            velocity: Vec3::default(),
        })
        .insert(Transform::from_translation(Vec3::new(0., 0., 0.)));

    let mut rng = thread_rng();
    for _ in 0..26 {
        let left: f32 = rng.gen_range(-1.0..1.0) * 50.;
        let bottom: f32 = rng.gen_range(-1.0..1.0) * 50.;

        let velx = rng.gen_range(-1.0..1.) * 5.;
        let vely = rng.gen_range(-1.0..1.) * 5.;
        let letter = rng.gen_range('a'..'z');

        spawn_boss_letter(
            &mut commands,
            &asset_server,
            letter,
            Vec3::new(velx, vely, 0.),
            left,
            bottom,
        );
    }
}

fn spawn_boss_letter(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    letter: char,
    velocity: Vec3,
    offset_left: f32,
    offset_bottom: f32,
) {
    commands
        .spawn()
        .insert_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(SCREEN_WIDTH * 0.5 + offset_left),
                    bottom: Val::Px(SCREEN_HEIGHT * 0.5 + offset_bottom),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
                sections: vec![TextSection {
                    value: letter.to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/OverpassMono-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                }],
            },
            ..Default::default()
        })
        .insert(BossLetter { velocity });
}

fn destroy_all_enemies(mut commands: Commands, enemies: Query<Entity, With<Enemy>>) {
    for entity in enemies.iter() {
        commands.entity(entity).despawn();
    }
}

fn boss_letter_swarm(
    time: Res<Time>,
    mut letters: Query<(&mut Style, &mut BossLetter)>,
    boss: Query<&Transform, With<Boss>>,
) {
    // subtract this to go from screen/bevy space to shape space
    let screen_to_shape: Vec3 = Vec3::new(SCREEN_WIDTH / 2., SCREEN_HEIGHT / 2., 0.);
    let mut rng = thread_rng();

    if let Ok(boss_transform) = boss.get_single() {
        for (mut style, mut letter) in letters.iter_mut() {
            let left = style.position.left;
            let left = match left {
                Val::Px(left) => left,
                _ => 0.,
            };
            let bottom = style.position.bottom;
            let bottom = match bottom {
                Val::Px(bottom) => bottom,
                _ => 0.,
            };
            let letter_location = Vec3::new(left, bottom, 0.);
            let letter_location = letter_location - screen_to_shape;

            let letter_to_target = boss_transform.translation - letter_location;

            style.position.bottom += letter.velocity.y;
            style.position.left += letter.velocity.x;

            letter.velocity += 0.5 * letter_to_target * time.delta_seconds();

            // this prevents letters from converging into a circle
            letter.velocity += Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.);

            // this prevents letters from getting further and further away
            letter.velocity = letter.velocity.clamp_length_max(10.);
        }
    } else {
        dbg!("NOT FOUND");
    }
}

fn boss_movement(time: Res<Time>, mut boss: Query<(&mut Transform, &mut Boss)>) {
    let (mut transform, mut boss) = boss.get_single_mut().unwrap();
    let mut rng = thread_rng();

    let velocity_change = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.);

    transform.translation += boss.velocity * time.delta_seconds();

    boss.velocity += velocity_change * 5.;

    // Keep the boss on screen (reversed and reduced to keep bounciness down)
    if transform.translation.x < -SCREEN_WIDTH / 2. {
        boss.velocity.x = f32::abs(boss.velocity.x) * 0.8;
    }
    if transform.translation.x > SCREEN_WIDTH / 2. {
        boss.velocity.x = f32::abs(boss.velocity.x) * -0.8;
    }
    if transform.translation.y < -SCREEN_HEIGHT / 2. {
        boss.velocity.y = f32::abs(boss.velocity.y) * 0.8;
    }
    if transform.translation.y > SCREEN_HEIGHT / 2. {
        boss.velocity.y = f32::abs(boss.velocity.y) * -0.8;
    }
}
