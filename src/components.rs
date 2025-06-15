use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Road;

#[derive(Component)]
pub struct RoadLine {
    pub speed: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub speed: f32,
}

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct GameOverStats;
