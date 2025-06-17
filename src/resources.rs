use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct GameSpeed {
    pub time_elapsed: f32,
    pub multiplier: f32,
}

impl Default for GameSpeed {
    fn default() -> Self {
        Self {
            time_elapsed: 0.0,
            multiplier: 1.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct GameOver(pub bool);

pub fn game_not_over(game_over: Res<GameOver>) -> bool {
    !game_over.0
}
