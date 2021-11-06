use crate::enemy::Enemy;
use crate::player::Player;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

fn collision_check_system(
    mut query_player: Query<(&Player, &Transform)>,
    mut query_enemy: Query<(&Enemy, &Transform)>,
) {
    for (player, transform) in query_player.iter() {
        for (enemy, transform_enemy) in query_enemy.iter() {
            todo!();
        }
    }
}
