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
    GreyPickupTruck,    
    TealSedan,          
    YellowTaxi,         
    RedConvertible,     
    GreenHatchback,     
    RedSportscar,       
    Player,             

}


impl CarType {
    // Custom collision bounds for each car type based on the assets
    pub fn collision_bounds(&self) -> Vec2 {
        match self {
            // Enemy cars 
            CarType::GreyPickupTruck => Vec2::new(56.0, 110.0),  
            CarType::TealSedan => Vec2::new(48.0, 95.0),         
            CarType::YellowTaxi => Vec2::new(48.0, 95.0),        
            CarType::RedConvertible => Vec2::new(44.0, 105.0),   
            CarType::GreenHatchback => Vec2::new(46.0, 85.0),    
            CarType::RedSportscar => Vec2::new(47.0,100.0),
            // Player car
            CarType::Player => Vec2::new(50.0, 120.0),            

        }
    }

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
