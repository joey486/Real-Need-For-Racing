use bevy::prelude::*;
use crate::components::{GameOverStats, GameOverUI};
use crate::resources::{GameOver, GameSpeed};

pub fn spawn_game_over_ui(mut commands: Commands,) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column, // Stack children vertically
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.8)),
                visibility: Visibility::Hidden,
                ..default()
            },
            GameOverUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Game Over",
                    TextStyle {
                        font_size: 60.0,
                        color: Color::RED,
                        ..default()
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            });

            // Stats
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Survival Time: 0.0s\nFinal Speed Multiplier: 0.0x\nDistance Traveled: 0m",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    style: Style {
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                },
                GameOverStats,
            ));

            // Instructions
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Press [R] to Restart or [Esc] to Quit",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::GRAY,
                        ..default()
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            });
        });
}

pub fn display_game_over_screen(
    game_over: Res<GameOver>,
    game_speed: Res<GameSpeed>,
    mut ui_query: Query<&mut Visibility, With<GameOverUI>>,
    mut stats_query: Query<&mut Text, With<GameOverStats>>,
) {
    if game_over.is_changed() && game_over.0 {
        // Show the game over UI
        if let Ok(mut visibility) = ui_query.get_single_mut() {
            *visibility = Visibility::Visible;
        }

        // Update stats text
        if let Ok(mut text) = stats_query.get_single_mut() {
            let survival_time = game_speed.time_elapsed;
            let final_speed = game_speed.multiplier;
            let distance = (survival_time * 50.0) as i32;
            
            text.sections[0].value = format!(
                "Survival Time: {:.1}s\nFinal Speed Multiplier: {:.1}x\nDistance Traveled: {}m",
                survival_time, final_speed, distance
            );
        }
    }
}

pub fn restart_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_over: ResMut<GameOver>,
    mut game_speed: ResMut<GameSpeed>,
    mut ui_query: Query<&mut Visibility, With<GameOverUI>>,
    enemies: Query<Entity, With<crate::components::Enemy>>,
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut crate::components::Velocity), With<crate::components::Player>>,
    mut enemy_timer: ResMut<crate::resources::EnemySpawnTimer>,
) {
    if game_over.0 {
        if keyboard_input.just_pressed(KeyCode::R) {
            // Reset game state
            game_over.0 = false;
            game_speed.time_elapsed = 0.0;
            game_speed.multiplier = 1.0;

            // Hide game over UI
            if let Ok(mut visibility) = ui_query.get_single_mut() {
                *visibility = Visibility::Hidden;
            }

            // Reset player position and velocity
            if let Ok((mut transform, mut velocity)) = player_query.get_single_mut() {
                transform.translation = Vec3::new(0.0, -200.0, 10.0);
                velocity.speed = 150.0;
            }

            // Remove all enemies
            for entity in enemies.iter() {
                commands.entity(entity).despawn();
            }

            // Reset enemy spawn timer
            enemy_timer.0.reset();

            println!("Game restarted!");
        } else if keyboard_input.just_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
    }
}