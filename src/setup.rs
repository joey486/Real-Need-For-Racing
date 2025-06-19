use bevy::prelude::*;
use bevy::asset::AssetServer;
use crate::components::*;
use crate::ui::spawn_game_over_ui;
use bevy_kira_audio::{Audio, AudioControl};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Road background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.2, 0.2),
                custom_size: Some(Vec2::new(800.0, 600.0)),
                ..default()
            },
            ..default()
        },
        Road,
    ));

    let line_texture = asset_server.load("white_line.png");
    for x in [-200.0, 0.0, 200.0] {
        super::systems::road::spawn_road_lines(&mut commands, line_texture.clone(), x, 10);
    }

    // Player with collision bounds
    let player_car_type = CarType::Player;
    let player_collision_bounds = player_car_type.collision_bounds();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(player_car_type.asset_path()),
            transform: Transform {
                translation: Vec3::new(0.0, -200.0, 10.0),
                scale: Vec3::splat(0.1),
                ..default()
            },
            ..default()
        },
        Player,
        Velocity { speed: 150.0 },
        player_car_type,
        CollisionBounds {
            size: player_collision_bounds,
        },
    ));

    // UI
    spawn_game_over_ui(commands);
}

pub fn play_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("audio/background_audio.ogg");
    audio.play(music).looped();
}
