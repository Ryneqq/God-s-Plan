mod app;
mod scene;
mod nodes;
mod world_map;

pub use scene::*;
pub use nodes::*;

pub mod consts {
    pub const WINDOW_HEIGHT: usize = 800;
    pub const WINDOW_WIDTH: usize = 800;
    pub const SCENE_TAIL_SIZE: usize = 16;
    pub const GRID_SIZE: usize = 2;
}

fn main() {
    app::run_app()
}
