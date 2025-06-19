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

#[derive(Component)]
pub struct Explosion {
    pub timer: Timer,
}

// Component to store collision bounds for each entity
#[derive(Component)]
pub struct CollisionBounds {
    pub size: Vec2,
}

// Car type enum to identify different car models
#[derive(Component, Clone, Copy, PartialEq)]
pub enum CarType {
    GreyPickupTruck,    // Image 1 - wider, longer truck
    TealSedan,          // Image 2 - standard sedan
    YellowTaxi,         // Image 3 - standard taxi
    RedConvertible,     // Image 4 - longer sports car
    GreenHatchback,     // Image 5 - compact car
    RedSportscar,       // Image 6 - sports car with spoiler
    Player,             // player.png

}


impl CarType {
    // Define custom collision bounds for each car type based on the images
    pub fn collision_bounds(&self) -> Vec2 {
        match self {
            // Enemy cars use scale 0.2, so we need larger base collision bounds
            CarType::GreyPickupTruck => Vec2::new(280.0, 550.0),  // 56*5, 110*5 - wider truck
            CarType::TealSedan => Vec2::new(240.0, 475.0),        // 48*5, 95*5 - standard sedan
            CarType::YellowTaxi => Vec2::new(240.0, 475.0),       // 48*5, 95*5 - similar to sedan
            CarType::RedConvertible => Vec2::new(220.0, 525.0),   // 44*5, 105*5 - longer sports car
            CarType::GreenHatchback => Vec2::new(230.0, 425.0),   // 46*5, 85*5 - smaller compact car
            CarType::RedSportscar => Vec2::new(240.0, 500.0),
            // Player uses scale 0.1, so we need even larger base collision bounds
            CarType::Player => Vec2::new(500.0, 1200.0),           // 50*10, 90*10 - player car bounds
        }
    }

      // Get the asset path for each car type
    pub fn asset_path(&self) -> &'static str {
        match self {
            CarType::GreyPickupTruck => "enemies/enemy1.png",
            CarType::TealSedan => "enemies/enemy2.png",
            CarType::YellowTaxi => "enemies/enemy3.png",
            CarType::RedConvertible => "enemies/enemy4.png",
            CarType::GreenHatchback => "enemies/enemy5.png",
            CarType::RedSportscar => "enemies/enemy6.png",
            CarType::Player => "player.png",
        }
    }

    // Map enemy index to car type
    pub fn from_enemy_index(index: u32) -> Self {
        match index {
            1 => CarType::GreyPickupTruck,
            2 => CarType::TealSedan,
            3 => CarType::YellowTaxi,
            4 => CarType::RedConvertible,
            5 => CarType::GreenHatchback,
            6 => CarType::RedSportscar,
            _ => CarType::TealSedan, // Default fallback
        }
    }
}
