use bevy::prelude::*;
use crate::consts::*;
use crate::WorldMap;

mod setup;
mod mouse;
mod update;

pub fn run_app() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "God's Plan".to_string(),
            width: WINDOW_WIDTH as f32,
            height: WINDOW_HEIGHT as f32,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(mouse::MousePosition::default())
        .add_resource(WorldMap::new(SCENE_TILE_SIZE))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup::setup.system())
        .add_system(mouse::mouse_position.system())
        .add_system(mouse::mouse_click.system())
        .add_system(mouse::mouse_scroll.system())
        .add_system(update::update.system())
        .run();
}
