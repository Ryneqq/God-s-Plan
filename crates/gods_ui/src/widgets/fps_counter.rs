use bevy::prelude::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy_egui::{egui, EguiContext};
use bevy::render::camera::Camera;


pub struct GodsFpsCounter;


impl Plugin for GodsFpsCounter {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_system(fps_counter.system());
    }
}


pub fn fps_counter(
    mut egui_context: ResMut<EguiContext>,
    diagnostics: Res<Diagnostics>,
) {
    let ctx = &mut egui_context.ctx;

    egui::TopPanel::top("top_panel").show(ctx, |ui| {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                let fps_value = format!("FPS: {:.2}", average);
                ui.label(fps_value);
            }
        }
    });
}