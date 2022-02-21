use crate::game_plugin::GameState;
use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;
use crate::TRAY_SIZE;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::game_plugin::actions::KeyActions;
use crate::game_plugin::SystemLabels::MoveEnemies;
use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use rand::{thread_rng, Rng};

use super::player::spawn_particle_burst;
use super::PlayInfo;
use super::PlayState;
use super::PHI;

const KILL_LINE_Y: f32 = -150.;
const ENEMY_FALL_SPEED: f32 = 20.0;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    pub letter: char,
}

#[derive(Component)]
pub struct KillLine;

#[derive(Component)]
pub struct EnemyDeathParticle {
    pub velocity: Vec3,
}

#[derive(Default)]
pub struct EnemySpawnTimer {
    time_since_last_spawn: f32,
    last_spawn_location: f32,
}
// TODO: keep track of time that is spend _not_ paused, for enemy spawning purposes

//fn run_if_not_paused(play_info: Res<PlayInfo>) -> ShouldRun {
//    match play_info.state {
//        PlayState::BossBattle => ShouldRun::Yes,
//        PlayState::Running => ShouldRun::Yes,
//        PlayState::Finished => ShouldRun::No,
//        PlayState::HitPaused => ShouldRun::No,
//    }
//}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_system_set(SystemSet::on_enter(GameState::Playing))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    //SystemSet::on_update(PlayState::Running)
                    .with_system(
                        move_enemy
                            //.with_run_criteria(run_if_not_paused)
                            .label(MoveEnemies),
                    )
                    //.with_system(check_lose.after(MoveEnemies))
                    .with_system(enemy_spawner), //.with_system(check_lose.after(MoveEnemies)),
            )
            .add_startup_system(startup_kill_line)
            .add_system(update_enemy_death_particles)
            .add_system_to_stage("resolve", check_lose);
    }
}

/// move and shrink the particles, and remove them when they're too small
fn update_enemy_death_particles(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &EnemyDeathParticle)>,
) {
    for (entity, mut transform, particle) in query.iter_mut() {
        transform.translation += particle.velocity * time.delta_seconds();
        transform.scale *= 0.9f32;

        if transform.scale.x < 0.1 {
            commands.entity(entity).despawn();
        }
    }
}

fn startup_kill_line(mut commands: Commands) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(800., 3.),
        origin: RectangleOrigin::Center,
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode {
                options: Default::default(),
                color: Default::default(),
            }),
            Transform::from_translation(Vec3::new(0., KILL_LINE_Y, 0.)),
        ))
        .insert(KillLine);
}

fn enemy_spawner(
    time: Res<Time>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
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

    // keep track of how long it's been since spawning an enemy, then spawn a new one if it's past the threshold and reset the timer.
    if enemy_spawn_timer.time_since_last_spawn >= spawn_period {
        let letter = letter_weights[dist.sample(&mut rng)].0;

        let screen_percent =
            (enemy_spawn_timer.last_spawn_location + PHI + rng.gen_range(-0.1f32..0.1f32)) % 1.0;
        enemy_spawn_timer.last_spawn_location = screen_percent;

        enemy_spawn_timer.time_since_last_spawn -= spawn_period;

        // add buffer to screen percent, so enemies don't spawn too close to the edges
        let buffer_percent = 0.1f32;
        let screen_percent = (screen_percent * (1f32 - 2f32 * buffer_percent)) + buffer_percent;

        commands
            .spawn()
            .insert_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(SCREEN_WIDTH * screen_percent),
                        bottom: Val::Px(SCREEN_HEIGHT * 3. / 4.),
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
                        value: format!("{}", letter.to_uppercase()),
                        style: TextStyle {
                            font: asset_server.load("fonts/OverpassMono-Bold.ttf"),
                            font_size: 120.0,
                            color: Color::WHITE,
                        },
                    }],
                },
                ..Default::default()
            })
            .insert(Enemy { letter });
    }
}

fn move_enemy(
    time: Res<Time>,
    play_info: Res<PlayInfo>,
    mut movement_query: Query<&mut Style, With<Enemy>>,
) {
    match play_info.state {
        PlayState::BossBattle => {}
        PlayState::Running => {}
        PlayState::Finished => return,
        PlayState::HitPaused => return,
    }
    for mut style in movement_query.iter_mut() {
        style.position.bottom += -ENEMY_FALL_SPEED * time.delta_seconds();
    }
}

fn check_lose(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut key_actions: ResMut<KeyActions>,
    mut enemies: Query<(&Transform, Entity, &Enemy)>,
) {
    if *state.current() == GameState::PlayingLose {
        return;
    }

    if key_actions.char_stack.clone().len() > TRAY_SIZE {
        state.push(GameState::PlayingLose).unwrap();
        return;
    }

    for (enemy_transform, entity, enemy) in enemies.iter_mut() {
        // TODO: this logic might not be very precise, if font size changes, etc.
        // For some reason the enemy transform isn't getting set on the first frame, so we have to check for default here
        if enemy_transform.translation.y < -KILL_LINE_Y
            && enemy_transform.translation != Default::default()
        {
            // TODO: add the letter to the tray
            commands.entity(entity).despawn();
            key_actions.char_stack.push(enemy.letter);

            let screen_to_shape: Vec3 = Vec3::new(SCREEN_WIDTH / 2., SCREEN_HEIGHT / 2., 0.); // TODO: this shouldn't need to be here. Too weird.
            spawn_particle_burst(
                &mut commands,
                &(enemy_transform.translation - screen_to_shape),
                Color::WHITE,
            );
        }
    }
}
