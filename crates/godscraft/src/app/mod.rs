use bevy::prelude::*;
use bevy_mod_picking::*;
use crate::WorldMap;

mod setup;
mod mouse;
mod update;
pub mod config;

pub fn run_app(config: config::Config) {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "Godscraft".to_string(),
            width: config.window_width() as f32,
            height: config.window_height() as f32,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .add_resource(mouse::MousePosition::default())
        .add_resource(WorldMap::new(config.tile_radius(), config.map_size()))
        .add_resource(config)
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        // .add_plugin(DebugPickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_startup_system(setup::setup.system())
        .add_system(mouse::mouse_position.system())
        .add_system(mouse::mouse_click.system())
        .add_system(mouse::mouse_scroll.system())
        .add_system(mouse::mouse_drag.system())
        .add_system(update::update.system())
        .add_system(set_highlight_params.system()) // move to some location as `picker`
        .run();
}

fn set_highlight_params(mut highlight_params: ResMut<PickHighlightParams>) {
    highlight_params.set_hover_color(Color::rgb(1.0, 0.0, 0.0));
    highlight_params.set_selection_color(Color::rgb(0.0, 1.0, 0.0));
}

