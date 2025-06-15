use bevy::prelude::*;
use crate::components::RoadLine;
use crate::resources::GameSpeed;


pub fn spawn_road_lines(
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

pub fn scroll_road_lines(
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
