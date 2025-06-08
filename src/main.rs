use bevy::{prelude::*, window::PresentMode};
use bevy::text::TextStyle;
use bevy::ui::{AlignItems, JustifyContent, PositionType};
use rand::Rng;

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Road;

#[derive(Component)]
struct RoadLine {
    speed: f32,
}

#[derive(Component)]
struct Velocity {
    speed: f32,
}

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

#[derive(Resource)]
struct GameSpeed {
    time_elapsed: f32,
    base_speed: f32,
    multiplier: f32,
}

#[derive(Resource, Default)]
struct GameOver(bool);

#[derive(Component)]
struct GameOverText;

// Run criteria to stop updates when game is over
fn game_not_over(game_over: Res<GameOver>) -> bool {
    !game_over.0
}

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
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .insert_resource(GameSpeed {
            time_elapsed: 0.0,
            base_speed: 150.0,
            multiplier: 1.0,
        })
        .insert_resource(GameOver::default())
        .add_systems(Startup, setup)
        .add_systems(Update, display_game_over_text)
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Road background (asphalt)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.2, 0.2), // Dark gray for asphalt
                custom_size: Some(Vec2::new(800.0, 600.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Road,
    ));

    // Road lines (white dashed lines)
    let line_texture = asset_server.load("white_line.png"); // A simple white rectangle

    // Left lane line
    spawn_road_lines(&mut commands, line_texture.clone(), -200.0, 10);

    // Center lane line
    spawn_road_lines(&mut commands, line_texture.clone(), 0.0, 10);

    // Right lane line
    spawn_road_lines(&mut commands, line_texture.clone(), 200.0, 10);

    // Player sprite
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform {
                translation: Vec3::new(0.0, -200.0, 10.0), // Position player near bottom
                scale: Vec3::new(0.1, 0.1, 1.0),           // Make player smaller
                ..default()
            },
            ..default()
        },
        Player,
        Velocity { speed: 150.0 },
    ));

    // UI root node
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "GAME OVER",
                        TextStyle {
                            font: asset_server.load("fonts/FireSans-Bold.ttf"),
                            font_size: 80.0,
                            color: Color::RED,
                        },
                    ),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                GameOverText,
            ));
        });
}

fn spawn_road_lines(
    commands: &mut Commands,
    texture: Handle<Image>,
    x_position: f32,
    count: usize,
) {
    let line_spacing = 150.0; // Space between lines
    let start_y = (count as f32 * line_spacing) / 2.0; // Start above screen

    for i in 0..count {
        let y_position = start_y - (i as f32 * line_spacing);

        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 50.0)), // Thin, short line
                    ..default()
                },
                transform: Transform::from_xyz(x_position, y_position, 1.0),
                ..default()
            },
            RoadLine { speed: 150.0 },
        ));
    }
}

fn spawn_enemy_over_time(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let enemy_index = rng.gen_range(1..=5);
        let x = rng.gen_range(-300.0..300.0);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(&format!("enemies/enemy{}.png", enemy_index)),
                transform: Transform {
                    translation: Vec3::new(x, 350.0, 10.0), // Spawn at top of window
                    scale: Vec3::new(0.2, -0.2, 1.0),
                    ..default()
                },
                ..default()
            },
            Enemy,
            Velocity { speed: 200.0 },
        ));
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 0.2;
    }
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        direction.x += 0.2;
    }
    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        velocity.speed += 10.0;
    }

    // Normalize direction
    if direction != Vec3::ZERO {
        direction = direction.normalize();
    }

    // Move player using their speed
    transform.translation += direction * velocity.speed * time.delta_seconds();

    // Clamp to road bounds
    const ROAD_BOUNDS: f32 = 350.0;
    transform.translation.x = transform.translation.x.clamp(-ROAD_BOUNDS, ROAD_BOUNDS);
}

fn enemy_movement(
    mut enemies: Query<(&mut Transform, &Velocity), With<Enemy>>,
    time: Res<Time>,
    game_speed: Res<GameSpeed>,
) {
    for (mut transform, velocity) in enemies.iter_mut() {
        transform.translation.y -= velocity.speed * game_speed.multiplier * time.delta_seconds();
    }
}

fn scroll_road_lines(
    mut lines_query: Query<(&mut Transform, &RoadLine)>,
    time: Res<Time>,
    game_speed: Res<GameSpeed>,
) {
    for (mut transform, line) in lines_query.iter_mut() {
        transform.translation.y -= line.speed * game_speed.multiplier * time.delta_seconds();
        if transform.translation.y < -350.0 {
            transform.translation.y = 350.0;
        }
    }
}

fn update_game_speed(mut game_speed: ResMut<GameSpeed>, time: Res<Time>) {
    game_speed.time_elapsed += time.delta_seconds();

    // Increase multiplier gradually
    game_speed.multiplier = 1.0 + (game_speed.time_elapsed / 30.0).min(3.0); // max 4x after 90s
}

fn cleanup_enemies(mut commands: Commands, query: Query<(Entity, &Transform), With<Enemy>>) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < -350.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn check_collision(
    mut game_over: ResMut<GameOver>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if game_over.0 {
        return; // already game over
    }

    let player_transform = player_query.single();
    let player_pos = player_transform.translation;
    let player_size = Vec2::new(64.0, 64.0); // assuming 64px player scaled by 0.1

    for enemy_transform in enemy_query.iter() {
        let enemy_pos = enemy_transform.translation;
        let enemy_size = Vec2::new(64.0, 64.0) ; // assuming 64px enemy scaled by 0.2

        let collision = (player_pos.x - enemy_pos.x).abs() < (player_size.x + enemy_size.x) / 2.0
            && (player_pos.y - enemy_pos.y).abs() < (player_size.y + enemy_size.y) / 2.0;

        if collision {
            game_over.0 = true;
            println!("💥 Game Over! You crashed into an enemy!");
            break;
        }
    }
}

fn display_game_over_text(
    game_over: Res<GameOver>,
    mut query: Query<&mut Visibility, With<GameOverText>>,
) {
    if game_over.is_changed() && game_over.0 {
        if let Ok(mut visibility) = query.get_single_mut() {
            *visibility = Visibility::Visible;
        }
    }
}
