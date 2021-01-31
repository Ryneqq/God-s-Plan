use bevy::prelude::*;

pub struct GodsUI;

impl Plugin for GodsUI {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

pub fn setup(commands: &mut Commands) {
    commands
        .spawn(CameraUiBundle::default())
        .with(GodsUI);
}