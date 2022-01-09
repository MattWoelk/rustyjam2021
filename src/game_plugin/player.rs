use crate::game_plugin::actions::{Actions, KeyActions};
use crate::game_plugin::enemy::Enemy;
use crate::game_plugin::loading::TextureAtlases;
use crate::game_plugin::GameState;
use bevy::prelude::*;
use std::collections::HashSet;

pub struct PlayerPlugin;

#[derive(Default, Component)]
pub struct Player {
    pub shot_timer: f32,
    pub state: PlayerState,
}

#[derive(Debug, Component)]
pub enum PlayerState {
    ShootingBullets,
    ShootingLaser,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::ShootingBullets
    }
}

#[derive(Default, Component)]
pub struct Bullet {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct Laser;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player)
                .with_system(spawn_camera),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_player.after("gather_input"))
                .with_system(shoot_enemies_with_keypresses.after("gather_input"))
                .with_system(shoot.after("gather_input"))
                .with_system(bullet_movement.after("gather_input"))
                .with_system(laser_movement.after("gather_input")),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(mut commands: Commands, texture_atlases: Res<TextureAtlases>) {
    let texture_atlas_handle = &texture_atlases.main_sprite_sheet;

    // Spawn Player
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            sprite: TextureAtlasSprite::new(188),
            ..Default::default()
        })
        .insert(Player::default())
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Transform::default())
                .insert(GlobalTransform::default())
                //.insert(Visible::default())
                .insert(Laser)
                .with_children(|laser_parent| {
                    laser_parent.spawn_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        transform: Transform {
                            translation: Vec3::new(0., 100., 0.),
                            rotation: Default::default(),
                            scale: Vec3::new(1., 10., 1.),
                        },
                        sprite: TextureAtlasSprite::new(189),
                        ..Default::default()
                    });
                });
        });
}

fn shoot_enemies_with_keypresses(key_actions: ResMut<KeyActions>, mut enemies: Query<&mut Enemy>) {
    let keys_pressed = key_actions.keys_just_pressed.clone();
    // TODO: this should be a set

    for mut enemy in enemies.iter_mut() {
        if keys_pressed.contains(&enemy.letter) {
            // TODO: remove it from the set
        }
    }

    let stack = key_actions.char_stack.iter().collect::<String>();

    dbg!(&stack, find_ending_word(&stack, &key_actions.all_words));

    // TODO: see if we spelt a word, and make a thing happen in that case? but that won't all happen in the same frame, so we need to keep track ...
    //       and we'll need to show on screen what was typed and in what order, so people can see if there's a word there.

    // TODO: idea: it's always checking the last thing you typed, and it clears those letters if they spell a word
    //             your goal is to get rid of your whole stack, which gives you a boost of some sort.
}

fn find_ending_word(text: &str, all_words: &HashSet<String>) -> Option<usize> {
    if text.len() < 3 {
        return None;
    }

    // TODO: is there a fast way to do this? Maybe if the word lists were split into lengths, and sorted? Or put in a set each?
    for length in (3..=text.len()).rev() {
        let substring = text
            .chars()
            .rev()
            .take(length)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();

        dbg!(&substring);

        if all_words.contains(&substring) {
            return Some(length);
        }
    }

    None
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in player_query.iter_mut() {
        player_transform.translation += movement;
    }
}

fn shoot(
    mut commands: Commands,
    time: Res<Time>,
    actions: Res<Actions>,
    texture_atlases: ResMut<TextureAtlases>,
    //mut query: Query<(&Transform, &mut Player)>,
    mut query: Query<(&Transform, &mut Player, &Children)>,
    mut q_laser: Query<(&mut Laser, &Children)>,
    //mut q_laser_sprite: Query<&mut Visible>,
) {
    let shot_delay = 0.2f32;

    for (_, mut player, _) in query.iter_mut() {
        player.shot_timer += time.delta().as_secs_f32();

        if actions.player_switch_weapon {
            player.state = match player.state {
                PlayerState::ShootingBullets => PlayerState::ShootingLaser,
                PlayerState::ShootingLaser => PlayerState::ShootingBullets,
            }
        }
    }

    if actions.player_shoot {
        let texture_atlas_handle = &texture_atlases.main_sprite_sheet;

        for (transform, mut player, children) in query.iter_mut() {
            match player.state {
                PlayerState::ShootingBullets => shoot_bullet_spray(
                    &mut player,
                    transform,
                    &mut commands,
                    shot_delay,
                    texture_atlas_handle,
                ), // TODO: also, delete/hide the laser ... maybe in a different system
                PlayerState::ShootingLaser => {
                    dbg!("lasors!");
                    for &child in children.iter() {
                        let (mut laser, mut children) = q_laser.get_mut(child).unwrap();
                        for &child in children.iter() {
                            //let mut visible = q_laser_sprite.get_mut(child).unwrap();
                            //visible.is_visible = false;
                        }
                    }

                    // TODO: if there is no laser, make one (should this be elsewhere, and just have it hidden?)
                    // TODO: try holding the Visible handle on the laser, so I don't have to do this hierarchy stuff
                }
            }
        }
    }
}

fn shoot_bullet_spray(
    player: &mut Player,
    transform: &Transform,
    commands: &mut Commands,
    shot_delay: f32,
    texture_atlas_handle: &Handle<TextureAtlas>,
) {
    if player.shot_timer > 0. {
        while player.shot_timer > 0. {
            player.shot_timer -= shot_delay;
        }
    } else {
        return;
    }

    let bullet_spread_directions = [
        Vec3::new(0.5, 0.5, 0.).normalize(),
        Vec3::new(0.0, 1., 0.).normalize(),
        Vec3::new(-0.5, 0.5, 0.).normalize(),
    ];
    for dir in bullet_spread_directions {
        commands
            .spawn()
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: transform.clone(),
                sprite: TextureAtlasSprite::new(188 - 24),
                ..Default::default()
            })
            .insert(Bullet { direction: dir });
    }
}

fn bullet_movement(mut commands: Commands, mut query: Query<(Entity, &mut Transform, &Bullet)>) {
    for (entity, mut transform, bullet) in query.iter_mut() {
        transform.translation += bullet.direction * 16.;

        if transform.translation.y > 280. {
            commands.entity(entity).despawn();
        }
    }
}

fn laser_movement(
    mut q_player: Query<(&Player, &Children)>,
    mut q_laser: Query<(&mut Laser, &mut Transform)>,
) {
    for (player, children) in q_player.iter_mut() {
        for &child in children.iter() {
            let (mut laser, mut transform) = q_laser.get_mut(child).unwrap();
            transform.rotate(Quat::from_rotation_z(0.01));
        }
    }
}
