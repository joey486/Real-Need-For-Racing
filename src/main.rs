mod components;
mod resources;
mod setup;
mod systems;
mod ui;

use bevy::{prelude::*, window::PresentMode};
use resources::*;
use setup::setup;
use ui::{display_game_over_screen, restart_game};
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Real Need For Racing".into(),
                resolution: (800., 600.).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(EnemySpawnTimer::default())
        .insert_resource(GameSpeed::default())
        .insert_resource(GameOver::default())
        .add_systems(Startup, setup)
        .add_systems(Update, display_game_over_screen)
        .add_systems(Update, restart_game)
        .add_systems(
            Update,
            (
                update_game_speed,
                player_movement,
                scroll_road_lines,
                enemy_movement,
                spawn_enemy_over_time,
                cleanup_enemies,
                check_collision,
            )
            .run_if(game_not_over),
        )
        .run();
}
