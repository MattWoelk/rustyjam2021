use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct EnemyPlugin;
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_enemy.system())
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_enemy.system()));
    }
}

fn spawn_enemy(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = textures.texture_tileset.clone().into();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 24, 10);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            sprite: TextureAtlasSprite::new(189),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(Enemy);
}

fn move_enemy(
    time: Res<Time>,
    mut movement_query: Query<&mut Transform, With<Enemy>>,
    mut sprite_query: Query<(&mut Timer, &mut TextureAtlasSprite)>,
) {
    for mut transform in movement_query.iter_mut() {
        transform.translation += Vec3::new(0., 4., 0.);
    }

    // rapidly swap its texture, like it's an animation or something.
    let anim_sprite_sheet_indices: [u32; 2] = [189, (189-24)];
    for (mut timer, mut sprite) in sprite_query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let current_index = anim_sprite_sheet_indices.iter().position(|&x| x == sprite.index).unwrap();
            sprite.index = anim_sprite_sheet_indices[(current_index + 1) % anim_sprite_sheet_indices.len()];
        }
    }
}