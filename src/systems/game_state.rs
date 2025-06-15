use bevy::prelude::*;
use crate::resources::GameSpeed;

pub fn update_game_speed(mut game_speed: ResMut<GameSpeed>, time: Res<Time>) {
    game_speed.time_elapsed += time.delta_seconds();

    // Increase multiplier gradually
    game_speed.multiplier = 1.0 + (game_speed.time_elapsed / 30.0).min(3.0); // max 4x after 90s
}
