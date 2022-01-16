use crate::game_plugin::loading::TextureAssets;
use crate::game_plugin::GameState;
use bevy::prelude::*;

use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use rand::thread_rng;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    pub(crate) letter: char,
}

#[derive(Default)]
pub struct EnemySpawnTimer {
    time_since_last_spawn: f32,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_enemy))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_enemy)
                    .with_system(enemy_spawner),
            );
    }
}

fn enemy_spawner(
    time: Res<Time>,
    textures: Res<TextureAssets>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    let texture_handle = textures.texture_tileset.clone();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 24, 10);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let letter_weights = [
        ('a', 9),
        ('b', 2),
        ('c', 2),
        ('d', 4),
        ('e', 12),
        ('f', 2),
        ('g', 3),
        ('h', 2),
        ('i', 9),
        ('j', 1),
        ('k', 1),
        ('l', 4),
        ('m', 2),
        ('n', 6),
        ('o', 8),
        ('p', 2),
        ('q', 1),
        ('r', 6),
        ('s', 4),
        ('t', 6),
        ('u', 4),
        ('v', 2),
        ('w', 2),
        ('x', 1),
        ('y', 2),
        ('z', 1),
    ];
    let dist = WeightedIndex::new(letter_weights.iter().map(|i| i.1)).unwrap();
    let mut rng = thread_rng();

    // keep track of how much time is passed
    enemy_spawn_timer.time_since_last_spawn += time.delta_seconds();

    let spawn_period = 1.0f32;

    // TODO: keep track of how long it's been since spawning an enemy, then spawn a new one if it's past the threshold and reset the timer.
    if enemy_spawn_timer.time_since_last_spawn >= spawn_period {
        //dbg!("spawning", enemy_spawn_timer.time_since_last_spawn);

        enemy_spawn_timer.time_since_last_spawn -= spawn_period;
        commands
            .spawn()
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                sprite: TextureAtlasSprite::new(189),
                ..Default::default()
            })
            .insert(Timer::from_seconds(0.1, true))
            .insert(Enemy {
                letter: letter_weights[dist.sample(&mut rng)].0,
            });
    }
}

fn spawn_enemy(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = textures.texture_tileset.clone();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 24, 10);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            sprite: TextureAtlasSprite::new(189),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(Enemy { letter: 'a' });
}

fn move_enemy(
    time: Res<Time>,
    mut movement_query: Query<&mut Transform, With<Enemy>>,
    mut sprite_query: Query<(&mut Timer, &mut TextureAtlasSprite)>,
) {
    for mut transform in movement_query.iter_mut() {
        transform.translation += Vec3::new(0., -4., 0.);
    }

    // rapidly swap its texture, like it's an animation or something.
    let anim_sprite_sheet_indices: [usize; 2] = [189, (189 - 24)];
    for (mut timer, mut sprite) in sprite_query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let current_index = anim_sprite_sheet_indices
                .iter()
                .position(|&x| x == sprite.index)
                .unwrap();
            sprite.index =
                anim_sprite_sheet_indices[(current_index + 1) % anim_sprite_sheet_indices.len()];
        }
    }
}
