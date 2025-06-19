use bevy::prelude::*;
use rand::Rng;
use crate::components::{Enemy, Velocity, CarType, CollisionBounds};
use crate::resources::{EnemySpawnTimer, GameSpeed};


pub fn spawn_enemy_over_time(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::rng();
        let enemy_index = rng.random_range(1..=5);
        let x = rng.random_range(-300.0..300.0);

        let car_type = CarType::from_enemy_index(enemy_index);
        let collision_bounds = car_type.collision_bounds();

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(car_type.asset_path()),
                transform: Transform {
                    translation: Vec3::new(x, 350.0, 10.0),
                    scale: Vec3::new(0.2, -0.2, 1.0),
                    ..default()
                },
                ..default()
            },
            Enemy,
            Velocity { speed: 200.0 },
            car_type,
            CollisionBounds {
                size: collision_bounds,
            },
        ));
    }
}

pub fn enemy_movement(
    mut enemies: Query<(&mut Transform, &Velocity), With<Enemy>>,
    time: Res<Time>,
    game_speed: Res<GameSpeed>,
) {
    for (mut transform, velocity) in enemies.iter_mut() {
        transform.translation.y -= velocity.speed * game_speed.multiplier * time.delta_seconds();
    }
}
pub fn cleanup_enemies(mut commands: Commands, query: Query<(Entity, &Transform), With<Enemy>>) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < -350.0 {
            commands.entity(entity).despawn();
        }
    }
}