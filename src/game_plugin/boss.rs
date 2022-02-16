use crate::game_plugin::SystemLabels::GatherInput;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::{thread_rng, Rng};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use super::{actions::KeyActions, enemy::Enemy, GameState};

const BOSS_LETTER_FONT_SIZE: f32 = 60.0;

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

#[derive(Component)]
struct BossFloorWord {
    word: String,
}

fn spawn_boss(mut commands: Commands, asset_server: Res<AssetServer>) {
    let boss_body_shape = shapes::Circle {
        radius: 100.,
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
    // subtract this to go from screen/bevy space to shape space
    let screen_to_shape: Vec3 = Vec3::new(
        SCREEN_WIDTH / 2.,
        SCREEN_HEIGHT / 2. - (BOSS_LETTER_FONT_SIZE / 2.),
        0.,
    );
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
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
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
    action: Res<KeyActions>,
    mut floor_words: Query<(&BossFloorWord, &mut Text)>,
) {
    let string_to_match = action.char_stack.iter().collect::<String>();

    // Make letters yellow, and do things if you hit spacebar
    for (floor_word, mut text) in floor_words.iter_mut() {
        if floor_word.word.starts_with(&string_to_match) {
            text.sections[0].value = string_to_match.clone();
            text.sections[1].value = floor_word.word[string_to_match.len()..].to_string();
        } else {
            text.sections[0].value = "".to_string();
            text.sections[1].value = floor_word.word.to_string();
        }

        if action.space_pressed && floor_word.word == string_to_match {
            // TODO: reset action
            // TODO: spawn bullets or whatever they are
            todo!()
        }
    }
}
