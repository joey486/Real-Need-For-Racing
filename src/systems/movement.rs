use bevy::prelude::*;
use crate::components::{Player, Velocity};


pub fn player_movement(
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
