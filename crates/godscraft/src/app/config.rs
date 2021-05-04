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
    pub lights: Vec<LightConfig>,
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

    pub fn lights(&'_ self) -> impl Iterator<Item = (Vec3, Light)> + '_ {
        self.lights.iter().map(|light| (light.position, Light::from(light)))
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
    pub color: Option<Vec3>,
    pub depth: Option<Vec2>,
    pub fov: Option<f32>,
}

impl<'a> From<&'a LightConfig> for Light {
    fn from(light: &'a LightConfig) -> Self {
        let mut new_light = Self::default();

        if let Some(color) = light.color {
            let [r, g, b]: [f32; 3] = color.into();
            let color = Color::rgb(r, g, b);

            new_light.color = color;
        }

        if let Some(depth) = light.depth {
            let [f, t]: [f32; 2] = depth.into();
            let depth = f..t;

            new_light.depth = depth;
        }

        if let Some(fov) = light.fov {
            new_light.fov = fov
        }

        new_light
    }
}
