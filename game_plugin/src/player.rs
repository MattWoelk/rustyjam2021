use crate::actions::Actions;
use crate::loading::TextureAtlases;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Default)]
pub struct Player {
    pub shot_timer: f32,
    pub state: PlayerState,
}

pub enum PlayerState {
    ShootingBullets,
    ShootingLaser,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::ShootingBullets
    }
}

#[derive(Default)]
pub struct Laser {
    pub direction: Vec3,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player.system())
                .with_system(spawn_camera.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_player.system())
                .with_system(shoot.system())
                .with_system(laser_movement.system()),
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
        .insert(Player::default());
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
    mut query: Query<(&Transform, &mut Player)>,
) {
    let shot_delay = 0.2f32;

    for (_, mut player) in query.iter_mut() {
        player.shot_timer += time.delta().as_secs_f32();
    }

    if actions.player_shoot {
        let texture_atlas_handle = &texture_atlases.main_sprite_sheet;

        for (transform, mut player) in query.iter_mut() {
            match player.state {
                PlayerState::ShootingBullets => shoot_bullet_spray(
                    &mut player,
                    transform,
                    &mut commands,
                    shot_delay,
                    texture_atlas_handle,
                ),
                PlayerState::ShootingLaser => todo!(),
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
    for a in bullet_spread_directions {
        commands
            .spawn()
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: transform.clone(),
                sprite: TextureAtlasSprite::new(188 - 24),
                ..Default::default()
            })
            .insert(Laser { direction: a });
    }
}

fn laser_movement(mut commands: Commands, mut query: Query<(Entity, &mut Transform, &Laser)>) {
    for (entity, mut transform, laser) in query.iter_mut() {
        transform.translation += laser.direction * 16.;

        if transform.translation.y > 280. {
            commands.entity(entity).despawn();
        }
    }
}
