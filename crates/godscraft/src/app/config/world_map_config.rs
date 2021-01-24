use serde::{Serialize, Deserialize};
use bevy::prelude::{Vec3};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMapConfig {
    pub tile_radius: f32,
    pub tile_height: f32,
    pub highlight_tile_position: Vec3,
    pub highlight_defualt_color: Vec3,
    pub map_size: isize, // TODO: for future i have to keep here some boundries
    pub tiles: HashMap<Tiles, String>,
}

impl WorldMapConfig {
    pub fn get_mesh_path(&self, tile: Tiles) -> Option<&str> {
        self.tiles.get(&tile).map(|s| s.as_str())
    }
}

#[serde(rename_all="snake_case")]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Tiles {
    Highlight,
    Forest,
}