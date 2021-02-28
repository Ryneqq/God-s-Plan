mod widgets;

pub use widgets::{ GodsFpsCounter };

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

const BEVY_TEXTURE_ID: u64 = 0;

pub struct GodsUI;

impl Plugin for GodsUI {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(EguiPlugin)
            .add_system(update_ui_scale_factor.system())
            .add_system(window.system())
            .add_plugin(GodsFpsCounter);
    }
}

// fn load_assets(_world: &mut World, resources: &mut Resources) {
//     let mut egui_context = resources.get_mut::<EguiContext>().unwrap();
//     let asset_server = resources.get::<AssetServer>().unwrap();

//     let texture_handle = asset_server.load("ui/icons/icon.png");
//     egui_context.set_egui_texture(BEVY_TEXTURE_ID, texture_handle);
// }

fn update_ui_scale_factor(mut egui_settings: ResMut<EguiSettings>, windows: Res<Windows>) {
    if let Some(window) = windows.get_primary() {
        egui_settings.scale_factor = 1.0 / window.scale_factor();
    }
}

fn window(_world: &mut World, resources: &mut Resources) {
    let mut egui_ctx = resources.get_mut::<EguiContext>().unwrap();
    let ctx = &mut egui_ctx.ctx;

    egui::Window::new("Window").scroll(true).show(ctx, |ui| {
        ui.selectable_label(false, "Windows can be moved by dragging them.");
        ui.label("They are automatically sized based on contents.");
        ui.selectable_label(false, "You can turn on resizing and scrolling if you like.");
        ui.label("You would normally chose either panels OR windows.");
    });
}
