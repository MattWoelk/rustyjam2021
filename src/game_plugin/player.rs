use crate::game_plugin::actions::KeyActions;
use crate::game_plugin::enemy::{Enemy, EnemyDeathParticle};
use crate::game_plugin::GameState;
use crate::game_plugin::SystemLabels::{EvaluateInput, GatherInput};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, TRAY_SIZE};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::{thread_rng, Rng};
use std::f32::consts::TAU;

//use super::{PlayInfo, PlayState};

pub struct PlayerPlugin;

#[derive(Default, Component)]
pub struct Player {}

/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_camera))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(
                    shoot_enemies_with_keypresses
                        .after(GatherInput)
                        .label(EvaluateInput),
                ),
            )
            .add_system_to_stage("resolve", check_completed_word);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn shoot_enemies_with_keypresses(
    mut commands: Commands,
    key_actions: ResMut<KeyActions>,
    enemies: Query<(Entity, &Transform, &Enemy)>,
) {
    // subtract this to go from screen/bevy space to shape space
    let screen_to_shape: Vec3 = Vec3::new(SCREEN_WIDTH / 2., SCREEN_HEIGHT / 2., 0.);

    let keys_pressed = key_actions.keys_just_pressed.clone();
    // TODO: keys_pressed should be a set

    let mut lowest_enemy: Option<(Entity, &Transform)> = None;

    for (entity, transform, enemy) in enemies.iter() {
        if keys_pressed.contains(&enemy.letter) {
            lowest_enemy = if let Some((_, old_transform)) = lowest_enemy {
                if old_transform.translation.y > transform.translation.y {
                    Some((entity, transform))
                } else {
                    lowest_enemy
                }
            } else {
                Some((entity, transform))
            }
        }
    }

    if let Some((lowest_enemy, transform)) = lowest_enemy {
        commands.entity(lowest_enemy).despawn();

        spawn_particle_burst(
            &mut commands,
            &(transform.translation - screen_to_shape),
            Color::WHITE,
        );
    }
}

fn check_completed_word(
    mut commands: Commands,
    mut key_actions: ResMut<KeyActions>,
    //mut play_info: ResMut<PlayInfo>,
) {
    if key_actions.space_pressed {
        let found_word = &key_actions.longest_word_option.clone();

        if let Some(word) = found_word {
            let number_of_letters = word.len();
            let stack_len = key_actions.char_stack.len();
            let num_chars_to_keep = stack_len - number_of_letters;
            key_actions.char_stack.truncate(num_chars_to_keep);

            spawn_word_removal_particles(&mut commands, num_chars_to_keep, number_of_letters);

            key_actions.all_collected_words.push(word.to_string());

            // TODO: hit pause: pop that state, and unpop after 0.5s
            //play_info.state = PlayState::HitPaused;
        }
    }
}

fn spawn_word_removal_particles(
    commands: &mut Commands,
    stack_length: usize,
    chars_removed: usize,
) {
    let y = -SCREEN_HEIGHT / 2. + 75.;
    let half_letter = 30.0;
    for n in 0..chars_removed {
        let x = -SCREEN_WIDTH / 2. + half_letter + (stack_length + n) as f32 * half_letter * 2.;
        let location: Vec3 = Vec3::new(x, y, 0.);
        spawn_particle_burst(commands, &location, Color::YELLOW);
    }
}

fn spawn_particle_burst(commands: &mut Commands, location: &Vec3, color: Color) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(20., 20.),
        origin: RectangleOrigin::Center,
    };

    let mut rng = thread_rng();
    for _ in 0..TRAY_SIZE {
        let angle: f32 = rng.gen_range(0.0..TAU);
        let magnitude: f32 = rng.gen_range(60.0..600.0);

        let velocity = Vec3::new(magnitude * angle.cos(), magnitude * angle.sin(), 0.);

        let location = *location + (velocity / 15.);

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Fill(FillMode {
                    options: Default::default(),
                    color,
                }),
                Transform::from_translation(location),
            ))
            .insert(EnemyDeathParticle { velocity });
    }
}
