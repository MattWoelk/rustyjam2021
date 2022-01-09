use crate::game_plugin::loading::TextureAssets;
use crate::game_plugin::GameState;
use bevy::prelude::*;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    pub(crate) letter: char,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_enemy))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_enemy));
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
        transform.translation += Vec3::new(0., 4., 0.);
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
