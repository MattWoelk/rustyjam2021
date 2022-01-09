use crate::game_plugin::enemy::Enemy;
use crate::game_plugin::player::Player;
use bevy::prelude::*;

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
