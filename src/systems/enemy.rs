use bevy::prelude::*;
use rand::Rng;
use crate::components::{Enemy, Velocity};
use crate::resources::{EnemySpawnTimer, GameSpeed};


pub fn spawn_enemy_over_time(
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