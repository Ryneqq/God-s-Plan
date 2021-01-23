use serde::{Serialize, Deserialize};
use bevy::prelude::Vec3;

const DEFAULT_WINDOW_HEIGHT: usize = 800;
const DEFAULT_WINDOW_WIDTH: usize = 800;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub window: Option<WindowConfig>,
    pub world_map: WorldMapConfig,
}

impl Config {
    pub fn window_height(&self) -> usize {
        self.window.as_ref()
            .and_then(|w| w.height)
            .unwrap_or(DEFAULT_WINDOW_HEIGHT)
    }

    pub fn window_width(&self) -> usize {
        self.window.as_ref()
            .and_then(|w| w.width)
            .unwrap_or(DEFAULT_WINDOW_WIDTH)
    }

    pub fn tile_radius(&self) -> f32 {
        self.world_map.tile_radius
    }

    pub fn tile_height(&self) -> f32 {
        self.world_map.tile_height
    }

    pub fn highlight_tile_pos_x(&self) -> f32 {
        self.world_map.highlight_tile_pos_x
    }

    pub fn map_size(&self) -> isize {
        self.world_map.map_size
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub height: Option<usize>,
    pub width: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMapConfig { // TODO this should be other file, like path to other config file
    pub tile_radius: f32,
    pub tile_height: f32,
    pub highlight_tile_pos_x: f32,
    pub map_size: isize, // TODO: for future i have to keep here some boundries?
}