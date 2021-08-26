use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

pub struct Player;

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
                .with_system(shoot_bullet.system())
                .with_system(laser_movement.system()),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = textures.texture_tileset.clone().into();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 24, 10);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Spawn Player
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            sprite: TextureAtlasSprite::new(188),
            ..Default::default()
        })
        .insert(Player);
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

fn shoot_bullet(
    mut commands: Commands,
    actions: Res<Actions>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<(&Transform, With<Player>)>,
) {
    if actions.player_shoot {
        let texture_handle = textures.texture_tileset.clone().into();
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 24, 10);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        for (transform, _) in query.iter_mut() {
            let bullet_spread_directions = [
                Vec3::new(0.5, 0.5, 0.),
                Vec3::new(0.0, 1., 0.),
                Vec3::new(-0.5, 0.5, 0.),
            ];
            for a in bullet_spread_directions {
                commands
                    .spawn()
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        transform: transform.clone(),
                        sprite: TextureAtlasSprite::new(188),
                        ..Default::default()
                    })
                    .insert(Laser { direction: a });
            }
        }
    }
}

fn laser_movement(mut commands: Commands, mut query: Query<(Entity, &mut Transform, &Laser)>) {
    for (entity, mut transform, laser) in query.iter_mut() {
        transform.translation += laser.direction * 8.; // Vec3::new(0., 4., 0.);

        if transform.translation.y > 280. {
            commands.entity(entity).despawn();
        }
    }
}
