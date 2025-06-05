use bevy::{prelude::*, window::PresentMode};
use rand::Rng;

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Resource)]
struct EnemyList(Vec<Entity>);

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
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_movement,
                scroll_road_lines,
                enemy_movement,
                spawn_enemy_over_time,
                cleanup_enemies,
            ),
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
    ));
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
            RoadLine { speed: 300.0 },
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
        let enemy_index = rng.random_range(1..=5);
        let x = rng.random_range(-300.0..300.0);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(&format!("enemies/enemy{}.png", enemy_index)),
                transform: Transform {
                    translation: Vec3::new(x, 350.0, 10.0), // Spawn at top of window
                    scale: Vec3::new(0.1, 0.1, 1.0),
                    ..default()
                },
                ..default()
            },
            Enemy,
            Velocity { speed: 150.0 },
        ));
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }

    // Optional: limit player to road area
    const ROAD_BOUNDS: f32 = 350.0;

    if direction != Vec3::ZERO {
        direction = direction.normalize();
    }

    player_transform.translation += direction * 400.0 * time.delta_seconds();

    // Clamp player position to road bounds
    player_transform.translation.x = player_transform
        .translation
        .x
        .clamp(-ROAD_BOUNDS, ROAD_BOUNDS);
}

fn enemy_movement(mut enemies: Query<(&mut Transform, &Velocity), With<Enemy>>, time: Res<Time>) {
    for (mut transform, velocity) in enemies.iter_mut() {
        transform.translation.y -= velocity.speed * time.delta_seconds();
    }
}

fn scroll_road_lines(mut lines_query: Query<(&mut Transform, &RoadLine)>, time: Res<Time>) {
    for (mut transform, line) in lines_query.iter_mut() {
        // Move line down
        transform.translation.y -= line.speed * time.delta_seconds();

        // If line is off-screen at the bottom, move it to the top
        if transform.translation.y < -350.0 {
            transform.translation.y = 350.0;
        }
    }
}

fn cleanup_enemies(mut commands: Commands, query: Query<(Entity, &Transform), With<Enemy>>) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < -350.0 {
            commands.entity(entity).despawn();
        }
    }
}
