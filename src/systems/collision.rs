use bevy::prelude::*;
use crate::components::{Player, Enemy};
use crate::resources::GameOver;

pub fn check_collision(
    mut game_over: ResMut<GameOver>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if game_over.0 {
        return; // already game over
    }

    let player_transform = player_query.single();
    let player_pos = player_transform.translation;
    let player_size = Vec2::new(64.0, 128.0); // assuming 64px player scaled by 0.1

    for enemy_transform in enemy_query.iter() {
        let enemy_pos = enemy_transform.translation;
        let enemy_size = Vec2::new(64.0, 128.0) ; // assuming 64px enemy scaled by 0.2

        let collision = (player_pos.x - enemy_pos.x).abs() < (player_size.x + enemy_size.x) / 2.0
            && (player_pos.y - enemy_pos.y).abs() < (player_size.y + enemy_size.y) / 2.0;

        if collision {
            game_over.0 = true;
            println!("Game Over! You crashed into an enemy!");
            break;
        }
    }
}
