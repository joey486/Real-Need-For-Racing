use crate::components::{Enemy, Explosion, Player, CollisionBounds};
use crate::resources::GameOver;
use bevy::prelude::*;


pub fn check_collision(
    mut commands: Commands,
    mut game_over: ResMut<GameOver>,
    player_query: Query<(&Transform, &CollisionBounds), With<Player>>,
    enemy_query: Query<(&Transform, &CollisionBounds), With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    if game_over.0 {
        return; // already game over
    }

    let Ok((player_transform, player_bounds)) = player_query.get_single() else {
        return; // No player found
    };

    let player_pos = player_transform.translation;
    let player_size = player_bounds.size;

    for (enemy_transform, enemy_bounds) in enemy_query.iter() {
        let enemy_pos = enemy_transform.translation;
        let enemy_size = enemy_bounds.size;

        // AABB (Axis-Aligned Bounding Box) collision detection
        let collision = aabb_collision(player_pos.truncate(), player_size, enemy_pos.truncate(), enemy_size);

        if collision {
            game_over.0 = true;

            // Spawn explosion at collision point
            let explosion_pos = (player_pos + enemy_pos) / 2.0;
            spawn_explosion(&mut commands, &asset_server, explosion_pos);

            break;
        }
    }
}

// AABB collision detection
fn aabb_collision(pos1: Vec2, size1: Vec2, pos2: Vec2, size2: Vec2) -> bool {
    let half_size1 = size1 / 2.0;
    let half_size2 = size2 / 2.0;

    let left1 = pos1.x - half_size1.x;
    let right1 = pos1.x + half_size1.x;
    let top1 = pos1.y + half_size1.y;
    let bottom1 = pos1.y - half_size1.y;

    let left2 = pos2.x - half_size2.x;
    let right2 = pos2.x + half_size2.x;
    let top2 = pos2.y + half_size2.y;
    let bottom2 = pos2.y - half_size2.y;

    // Check if rectangles overlap
    !(left1 >= right2 || right1 <= left2 || top1 <= bottom2 || bottom1 >= top2)
}

fn spawn_explosion(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
) {
    let texture_handle = asset_server.load("explosion.png");

    commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            transform: Transform {
                translation: position,
                scale: Vec3::new(0.05, 0.05, 0.1),
                ..Default::default()
            },
            ..Default::default()
        },
        Explosion {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        },
    ));
}

pub fn explosion_cleanup_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Explosion)>,
) {
    for (entity, mut explosion) in query.iter_mut() {
        explosion.timer.tick(time.delta());
        if explosion.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[allow(dead_code)]
// Visual debug system to show collision bounds (for development ONLY)
#[cfg(debug_assertions)]
pub fn debug_draw_collision_bounds(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &CollisionBounds)>,
) {
    for (transform, bounds) in query.iter() {
        let pos = transform.translation.truncate();
        let size = bounds.size;
        
        // Draw a rectangle outline around the collision bounds
        gizmos.rect_2d(pos, 0.0, size, Color::RED);
    }
}


// // System to update collision bounds if car types change during runtime
// pub fn update_collision_bounds(
//     mut query: Query<(&CarType, &mut CollisionBounds), Changed<CarType>>,
// ) {
//     for (car_type, mut bounds) in query.iter_mut() {
//         bounds.size = car_type.collision_bounds();
//     }
// }