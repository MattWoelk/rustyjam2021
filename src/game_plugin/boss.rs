use crate::game_plugin::SystemLabels::GatherInput;
use bevy::{core::FixedTimestep, prelude::*};
use bevy_prototype_lyon::prelude::*;
use rand::{thread_rng, Rng};
use std::f32::consts::TAU;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use super::{actions::KeyActions, enemy::Enemy, GameState};

const BOSS_LETTER_FONT_SIZE: f32 = 60.;
const BOSS_RADIUS: f32 = 100.;
const BOSS_STARTING_HEALTH: u32 = 26;

pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Boss)
                .with_system(destroy_all_enemies)
                .with_system(reset_stack)
                .with_system(spawn_words_on_the_ground)
                .with_system(spawn_boss),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Boss)
                .with_system(boss_letter_swarm)
                .with_system(set_keyboard_actions_boss_mode.label(GatherInput))
                .with_system(update_floor_words.after(GatherInput))
                .with_system(check_boss_letter_bullet_overlaps),
        )
        // TODO: these are being run even when we're not in GameState::Boss. :(
        .add_system_set(
            SystemSet::on_update(GameState::Boss)
                .with_run_criteria(FixedTimestep::step(1. / 60.))
                .with_system(boss_movement)
                .with_system(letter_bullet_movement),
        );
    }
}

#[derive(Component)]
pub struct Boss {
    velocity: Vec3,
    health: u32,
}

#[derive(Component)]
struct BossLetter {
    velocity: Vec3,
}

#[derive(Component)]
struct BossFloorWord {
    word: String,
}

#[derive(Component)]
struct LetterBullet {
    velocity: Vec3,
    time_left: f32,
    mode: LetterBulletMode,
}

#[derive(Clone)]
enum LetterBulletMode {
    Straight,
    StraightWithDrag(f32), // drag value
}

fn spawn_boss(mut commands: Commands, asset_server: Res<AssetServer>) {
    let boss_body_shape = shapes::Circle {
        radius: BOSS_RADIUS,
        center: Default::default(),
    };
    commands
        //.spawn()
        .spawn_bundle(GeometryBuilder::build_as(
            &boss_body_shape,
            DrawMode::Fill(FillMode {
                options: Default::default(),
                color: Color::DARK_GRAY,
            }),
            Transform::default(),
        ))
        .insert(Boss {
            velocity: Vec3::default(),
            health: BOSS_STARTING_HEALTH,
        })
        .insert(Transform::from_translation(Vec3::new(0., 0., 0.)));

    let mut rng = thread_rng();
    for _ in 0..BOSS_STARTING_HEALTH {
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
                        font_size: BOSS_LETTER_FONT_SIZE,
                        color: Color::WHITE,
                    },
                }],
            },
            ..Default::default()
        })
        .insert(BossLetter { velocity });
}

fn reset_stack(mut action: ResMut<KeyActions>) {
    action.char_stack.clear();
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
    let mut rng = thread_rng();

    if let Ok(boss_transform) = boss.get_single() {
        for (mut style, mut letter) in letters.iter_mut() {
            let letter_location = style_to_position(&style);

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
    let boss = match boss.get_single_mut() {
        Ok(boss) => boss,
        Err(_) => return,
    };
    let (mut transform, mut boss) = boss;
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

fn spawn_words_on_the_ground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    key_actions: Res<KeyActions>,
) {
    let mut words = &key_actions.all_collected_words;
    // TODO: make sure there are no duplicates in the words.
    // TODO: keep the longest X words

    // TODO: have this only work in debug mode.
    let backup_words = vec![
        "the".to_string(),
        "quick".to_string(),
        "brown".to_string(),
        "fox".to_string(),
        "jumps".to_string(),
        "over".to_string(),
        "the".to_string(),
        "lazy".to_string(),
        "dog".to_string(),
    ];

    if words.is_empty() {
        words = &backup_words;
    }

    let mut rng = thread_rng();

    for word in words {
        let x = rng.gen_range(0.0..1.0);
        let y = rng.gen_range(0.0..1.0);

        let left = SCREEN_WIDTH * x;
        let bottom = SCREEN_HEIGHT * y;

        commands
            .spawn()
            .insert_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(left),
                        bottom: Val::Px(bottom),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                    sections: vec![
                        TextSection {
                            value: ".".to_string().to_uppercase(),
                            style: TextStyle {
                                font: asset_server.load("fonts/OverpassMono-Bold.ttf"),
                                font_size: 60.,
                                color: Color::YELLOW,
                            },
                        },
                        TextSection {
                            value: word.to_string().to_uppercase(),
                            style: TextStyle {
                                font: asset_server.load("fonts/OverpassMono-Bold.ttf"),
                                font_size: 60.,
                                color: Color::WHITE,
                            },
                        },
                    ],
                },
                ..Default::default()
            })
            .insert(BossFloorWord {
                word: word.to_string(),
            });
    }
}

fn set_keyboard_actions_boss_mode(
    mut action: ResMut<KeyActions>,
    keyboard_input: Res<Input<KeyCode>>,
    floor_words: Query<&BossFloorWord>,
) {
    let new_chars: Vec<char> = keyboard_input
        .get_just_pressed()
        .filter_map(|code: &bevy::prelude::KeyCode| match code {
            KeyCode::A => Some('a'),
            KeyCode::B => Some('b'),
            KeyCode::C => Some('c'),
            KeyCode::D => Some('d'),
            KeyCode::E => Some('e'),
            KeyCode::F => Some('f'),
            KeyCode::G => Some('g'),
            KeyCode::H => Some('h'),
            KeyCode::I => Some('i'),
            KeyCode::J => Some('j'),
            KeyCode::K => Some('k'),
            KeyCode::L => Some('l'),
            KeyCode::M => Some('m'),
            KeyCode::N => Some('n'),
            KeyCode::O => Some('o'),
            KeyCode::P => Some('p'),
            KeyCode::Q => Some('q'),
            KeyCode::R => Some('r'),
            KeyCode::S => Some('s'),
            KeyCode::T => Some('t'),
            KeyCode::U => Some('u'),
            KeyCode::V => Some('v'),
            KeyCode::W => Some('w'),
            KeyCode::X => Some('x'),
            KeyCode::Y => Some('y'),
            KeyCode::Z => Some('z'),
            _ => None,
        })
        .collect();

    // Backspace to undo typing
    if keyboard_input.just_pressed(KeyCode::Back) {
        action.char_stack.pop();
    }

    let mut potential_word: String = action.char_stack.clone().iter().collect();
    potential_word += &mut new_chars.iter().collect::<String>();

    // If we found a match, update the stack
    if floor_words
        .iter()
        .any(|e| e.word.starts_with(&potential_word))
    {
        action.char_stack = potential_word.chars().collect();
    }

    action.new_press = !new_chars.is_empty();
    action.keys_just_pressed = new_chars;
    action.space_pressed = keyboard_input.just_pressed(KeyCode::Space);
}

fn update_floor_words(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut action: ResMut<KeyActions>,
    mut floor_words: Query<(&BossFloorWord, &mut Text, &Transform)>,
) {
    let string_to_match = action.char_stack.iter().collect::<String>();

    // Make letters yellow, and do things if you hit spacebar
    for (floor_word, mut text, transform) in floor_words.iter_mut() {
        if floor_word.word.starts_with(&string_to_match) {
            text.sections[0].value = string_to_match.clone();
            text.sections[1].value = floor_word.word[string_to_match.len()..].to_string();
        } else {
            text.sections[0].value = "".to_string();
            text.sections[1].value = floor_word.word.to_string();
        }

        if action.space_pressed && floor_word.word == string_to_match {
            // TODO: reset action
            // TODO: send in modifiers, if the word contains those certain letters

            action.char_stack.clear();

            for c in floor_word.word.chars() {
                spawn_floor_bullets(
                    &mut commands,
                    &asset_server,
                    c,
                    transform.translation.x,
                    transform.translation.y,
                    match c {
                        'x' | 'i' => 2.,
                        _ => 1.,
                    },
                );
            }
        }
    }
}

fn letter_bullet_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut movement_query: Query<(&mut Style, &mut LetterBullet, Entity)>,
) {
    for (mut style, mut letter, entity) in movement_query.iter_mut() {
        match letter.mode {
            LetterBulletMode::Straight => {
                let delta = letter.velocity * time.delta_seconds();

                style.position.left += delta.x;
                style.position.bottom += delta.y;
            }
            LetterBulletMode::StraightWithDrag(drag) => {
                let new_velocity = letter.velocity * (1. - drag);
                letter.velocity = new_velocity;

                let delta = new_velocity * time.delta_seconds();
                style.position.left += delta.x;
                style.position.bottom += delta.y;
            }
        }

        letter.time_left -= time.delta_seconds();

        if letter.time_left <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_floor_bullets(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    letter: char,
    offset_left: f32,
    offset_bottom: f32,
    duration: f32,
) {
    let info = match letter_to_bullet_info(letter) {
        Some(i) => i,
        None => return,
    };
    let positions_and_velocities = info.spread_style.random_positions_velocities(info.quantity);

    for (position, velocity) in positions_and_velocities {
        commands
            .spawn()
            .insert_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(/*SCREEN_WIDTH * 0.5 + */ offset_left + position.x),
                        bottom: Val::Px(/*SCREEN_HEIGHT * 0.5 + */ offset_bottom + position.y),
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
                        value: info.letter_display.to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/OverpassMono-Bold.ttf"),
                            font_size: BOSS_LETTER_FONT_SIZE / 2.,
                            color: Color::WHITE,
                        },
                    }],
                },
                ..Default::default()
            })
            .insert(LetterBullet {
                velocity,
                mode: info.bullet_mode.clone(),
                time_left: duration,
            });
    }
}

struct BulletInfo {
    quantity: usize,
    spread_style: SpreadStyle,
    letter_display: char,
    bullet_mode: LetterBulletMode,
}

enum SpreadStyle {
    Star,
    Circular,
    X,
}

impl SpreadStyle {
    fn random_positions_velocities(&self, quantity: usize) -> Vec<(Vec3, Vec3)> {
        let mut rng = thread_rng();
        match self {
            SpreadStyle::Circular => (0..quantity)
                .into_iter()
                .map(|_| {
                    let angle: f32 = rng.gen_range(0.0..TAU);
                    let magnitude: f32 = rng.gen_range(400.0..600.0);

                    let velocity = Vec3::new(magnitude * angle.cos(), magnitude * angle.sin(), 0.);

                    (Vec3::new(0., 0., 0.), velocity)
                })
                .collect(),
            SpreadStyle::Star => (0..quantity)
                .into_iter()
                .map(|i| {
                    let angle: f32 = i as f32 * TAU * (1. / 6.);
                    let magnitude: f32 = rng.gen_range(200.0..300.0);

                    let velocity = Vec3::new(magnitude * angle.cos(), magnitude * angle.sin(), 0.);

                    (Vec3::new(0., 0., 0.), velocity)
                })
                .collect(),
            SpreadStyle::X => (0..quantity)
                .into_iter()
                .map(|i| {
                    let angle: f32 = (i as f32 * (1. / 4.) + (1. / 8.)) * TAU;
                    let magnitude: f32 = rng.gen_range(200.0..300.0);

                    let velocity = Vec3::new(magnitude * angle.cos(), magnitude * angle.sin(), 0.);

                    (Vec3::new(0., 0., 0.), velocity)
                })
                .collect(),
        }
    }
}

fn letter_to_bullet_info(c: char) -> Option<BulletInfo> {
    // TODO: how many
    // TODO: what pattern
    // TODO: what velocity
    // TODO: starting positions
    use SpreadStyle::*;

    match c {
        'e' => Some(BulletInfo {
            quantity: 8,
            spread_style: Circular,
            letter_display: 'E',
            bullet_mode: LetterBulletMode::StraightWithDrag(0.05),
        }),
        'i' => Some(BulletInfo {
            quantity: 12,
            spread_style: Star,
            letter_display: 'I',
            bullet_mode: LetterBulletMode::Straight,
        }),
        'x' => Some(BulletInfo {
            quantity: 12,
            spread_style: X,
            letter_display: 'X',
            bullet_mode: LetterBulletMode::Straight,
        }),
        _ => None,
    }
}

fn check_boss_letter_bullet_overlaps(
    mut commands: Commands,
    mut boss: Query<(&Transform, &mut Boss)>,
    mut boss_letters: Query<Entity, With<BossLetter>>,
    movement_query: Query<(&Style, &LetterBullet, Entity)>,
) {
    let mut health_lost: u32 = 0;
    if let Ok((boss_transform, mut boss)) = boss.get_single_mut() {
        for (style, _, entity) in movement_query.iter() {
            // TODO: compare letter position to boss position. If it's within range, then damage the boss
            let pos = style_to_position(style);
            if pos.distance(boss_transform.translation) < BOSS_RADIUS {
                boss.health -= 1;
                health_lost += 1;
                dbg!(boss.health);
                commands.entity(entity).despawn();
            }
        }

        // delete boss letters based on how many bullets it took this frame.
        for entity in boss_letters.iter_mut() {
            if health_lost == 0 {
                break;
            }
            commands.entity(entity).despawn();
            health_lost -= 1;
        }
    }
}

fn style_to_position(style: &Style) -> Vec3 {
    let screen_to_shape: Vec3 = Vec3::new(
        SCREEN_WIDTH / 2.,
        SCREEN_HEIGHT / 2. - (BOSS_LETTER_FONT_SIZE / 2.),
        0.,
    );

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

    Vec3::new(left, bottom, 0.) - screen_to_shape
}
