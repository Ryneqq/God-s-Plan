mod app;
mod world_map;
mod hexagon;

pub use world_map::*;
pub use hexagon::*;

use std::fs;
use app::config::Config;

const CONFIG_PATH: &str = "crates/godscraft/assets/config/config.json";

// TODO: Add this as configuration file
pub mod consts {
    pub const WINDOW_HEIGHT: usize = 800;
    pub const WINDOW_WIDTH: usize = 800;
    pub const SCENE_TILE_SIZE: f32 = 1.04;
    pub const SCENE_SIZE: isize = 4;
    pub const TILE_HEIGHT: f32 = 0.2;
    pub const HIGHLIGHT_TILE_PLACEMENT_X: f32 = 0.19;
}

fn main() {
    let config = fs::read_to_string(CONFIG_PATH)
        .expect(&format!("Fatal error: Could not read config file under {}", CONFIG_PATH));
    let config: Config = serde_json::from_str(&config)
        .expect("Fatal error: Could not deserialize config file");

    app::run_app(config)
}
