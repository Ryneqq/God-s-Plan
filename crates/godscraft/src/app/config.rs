mod world_map_config;

pub use self::world_map_config::Tiles;
use self::world_map_config::WorldMapConfig;
use serde::{Serialize, Deserialize};
use bevy::prelude::{Vec2, Vec3, Color, Light};

const DEFAULT_WINDOW_HEIGHT: usize = 800;
const DEFAULT_WINDOW_WIDTH: usize = 800;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub window: Option<WindowConfig>,
    pub world_map: WorldMapConfig,
    pub camera: CameraConfig,
    pub light: LightConfig,
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

    pub fn highlight_tile_position(&self) -> Vec3 {
        self.world_map.highlight_tile_position
    }

    pub fn highlight_default_color(&self) -> Color {
        let [r, g, b]: [f32; 3] = self.world_map.highlight_defualt_color.into();

        Color::rgb(r, g, b)
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

    pub fn light_position(&self) -> Vec3 {
        self.light.position
    }

    pub fn light(&self) -> Light {
        Light::from(&self.light)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub height: Option<usize>,
    pub width: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub position: Vec3,
    pub angle: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightConfig {
    pub position: Vec3,
    pub color: Vec3,
    pub depth: Vec2,
    pub fov: f32,
}

impl<'a> From<&'a LightConfig> for Light {
    fn from(light: &'a LightConfig) -> Self {
        let [r, g, b]: [f32; 3] = light.color.into();
        let [f, t]: [f32; 2] = light.depth.into();
        let color = Color::rgb(r, g, b);
        let depth = f..t;
        let fov = light.fov;

        Self {color, depth, fov}
    }
}
