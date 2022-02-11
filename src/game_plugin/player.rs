use crate::game_plugin::actions::KeyActions;
use crate::game_plugin::enemy::{Enemy, EnemyDeathParticle};
use crate::game_plugin::GameState;
use crate::game_plugin::SystemLabels::{EvaluateInput, GatherInput};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::{thread_rng, Rng};
use std::f32::consts::TAU;

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
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn shoot_enemies_with_keypresses(
    mut commands: Commands,
    mut key_actions: ResMut<KeyActions>,
    mut enemies: Query<(Entity, &Transform, &mut Enemy)>,
) {
    // subtract this to go from screen/bevy space to shape space
    let screen_to_shape: Vec3 = Vec3::new(SCREEN_WIDTH / 2., SCREEN_HEIGHT / 2., 0.);

    let keys_pressed = key_actions.keys_just_pressed.clone();
    // TODO: keys_pressed should be a set

    let mut lowest_enemy: Option<(Entity, &Transform)> = None;

    for (entity, transform, enemy) in enemies.iter_mut() {
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

        let shape = shapes::Rectangle {
            extents: Vec2::new(20., 20.),
            origin: RectangleOrigin::Center,
        };

        let mut rng = thread_rng();
        for _ in 0..10 {
            let angle: f32 = rng.gen_range(0.0..TAU);
            let magnitude: f32 = rng.gen_range(60.0..600.0);

            let velocity = Vec3::new(magnitude * angle.cos(), magnitude * angle.sin(), 0.);

            let location: Vec3 = transform.translation - screen_to_shape;
            let location = location + (velocity / 15.);

            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Fill(FillMode {
                        options: Default::default(),
                        color: Default::default(),
                    }),
                    Transform::from_translation(location),
                ))
                .insert(EnemyDeathParticle { velocity });
        }
    }

    let found_word = &key_actions.longest_word_option;

    // TODO: should this be its own system? Probably.
    if key_actions.space_pressed {
        if let Some(word) = found_word {
            let number_of_letters = word.len();
            let stack_len = key_actions.char_stack.len();
            key_actions
                .char_stack
                .truncate(stack_len - number_of_letters);
        }
    }

    // TODO: see if we spelt a word, and make a thing happen in that case? but that won't all happen in the same frame, so we need to keep track ...
    //       and we'll need to show on screen what was typed and in what order, so people can see if there's a word there.

    // TODO: idea: it's always checking the last thing you typed, and it clears those letters if they spell a word
    //             your goal is to get rid of your whole stack, which gives you a boost of some sort.
}
