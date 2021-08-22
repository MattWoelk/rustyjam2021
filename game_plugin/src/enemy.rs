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
        .insert(Enemy);
}

fn move_enemy(
    mut query: Query<&mut Transform, With<Enemy>>,
) {
    for mut transform in query.iter_mut() {
        transform.translation += Vec3::new(0., 4., 0.);
    }
}