use bevy::prelude::*;

pub enum CollisionType {
    None,
    Solid,
    Platform,
}

pub enum DamageType {
    None,
    Hurt,
    Kill,
}

pub enum Collectible {
    None,
    Coin,
    Key,
}

pub struct Tile {
    pub identifier: String,
    pub collision: CollisionType,
    pub damage: DamageType,
    pub sprite_index: i32,
    pub script: Option<String>,
}

pub struct Level {
    pub identifier: String,
    pub checkpoints: Vec<UVec2>,
    pub spawn_point: UVec2,
    pub size: UVec2,          // Size in tiles
    pub sprites: Vec<String>, // List of paths to sprites for a level
    pub tiles: Vec<Tile>,
}
