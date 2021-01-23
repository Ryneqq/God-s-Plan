use serde::{Serialize, Deserialize};
use bevy::prelude::Vec3;

const DEFAULT_WINDOW_HEIGHT: usize = 800;
const DEFAULT_WINDOW_WIDTH: usize = 800;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub window: Option<WindowConfig>,
    pub world_map: WorldMapConfig,
    pub camera: Camera,
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

    pub fn camera_position(&self) -> Vec3 {
        self.camera.position
    }

    pub fn camera_angle(&self) -> f32 {
        self.camera.angle
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub height: Option<usize>,
    pub width: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMapConfig {
    pub tile_radius: f32,
    pub tile_height: f32,
    pub highlight_tile_pos_x: f32,
    pub map_size: isize, // TODO: for future i have to keep here some boundries?
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub position: Vec3,
    pub angle: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_serailization() {
        let camera = Camera {
            position: Vec3::zero(),
            angle: 1.0
        };
        let json = serde_json::to_string_pretty(&camera).unwrap();
        println!("{}", json);

        panic!();
    }
}