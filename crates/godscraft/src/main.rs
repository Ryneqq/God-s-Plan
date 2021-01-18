mod app;
mod world_map;
mod hexagon;

pub use world_map::*;
pub use hexagon::*;

pub mod consts {
    pub const WINDOW_HEIGHT: usize = 800;
    pub const WINDOW_WIDTH: usize = 800;
    pub const SCENE_TILE_SIZE: f32 = 1.04;
}

fn main() {
    app::run_app()
}
