#![windows_subsystem = "windows"]
mod components;
mod resources;
mod setup;
mod systems;
mod ui;

use bevy::{asset::AssetPlugin, prelude::*, window::PresentMode};
use resources::*;
use setup::setup;
use setup::play_music;
use systems::*;
use ui::{display_game_over_screen, restart_game};
use bevy_embedded_assets::EmbeddedAssetPlugin; 
use bevy_kira_audio::prelude::*;
use collision::explosion_cleanup_system;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Real Need For Racing".into(),
                        resolution: (800., 600.).into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    asset_folder: "assets".into(),
                    watch_for_changes: None,
                    ..default()
                }),AudioPlugin)
        )
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
                explosion_cleanup_system,
            )
                .run_if(game_not_over),
        )
        .add_systems(Startup, play_music) 
        .run();
}

/*
 For debugging to view bounding boxes:
 Add this to add_systems()
    #[cfg(debug_assertions)]
    debug_draw_collision_bounds, 
 */