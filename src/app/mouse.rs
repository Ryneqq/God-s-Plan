use crate::consts::*;
use bevy::{
    prelude::*,
    window::CursorMoved,
    input::mouse::MouseWheel,
    render::camera::Camera
};

#[derive(Default)]
pub struct MouseState {
    cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_wheel_event_reader: EventReader<MouseWheel>
}

#[derive(Debug, Default)]
pub struct MousePosition {
    pub position: Vec2
}

pub (super) fn mouse_position(
    mut mouse: ResMut<MousePosition>,
    mut state: Local<MouseState>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        mouse.position = event.position;
    }
}

pub (super) fn mouse_click(
    mouse_button: Res<Input<MouseButton>>,
    mouse_position: Res<MousePosition>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position.position;
        let _mouse_pos = Vec2::new(
            mouse_pos.x - WINDOW_WIDTH as f32 / 2.,
            mouse_pos.y - WINDOW_HEIGHT as f32 / 2.,
        );

        // TODO
    }
}

pub (super) fn mouse_scroll(
    mut state: Local<MouseState>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    mut camera_query: Query<(&Camera, &mut Transform)>
) {
    for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        for (_, mut transform) in camera_query.iter_mut() {
            let direction = quat_to_direction(transform.rotation);
            let scroll = event.y;
            transform.translation += direction * scroll;
        }
    }
}

fn quat_to_direction(quat: Quat) -> Vec3 {
    let [x, y, z, w] = quat.as_ref();

     Vec3::new(
        2f32 * (x*z + w*y),
        2f32 * (y*z - w*x),
        1f32 - 2f32 * (x*x + y*y),
     )
}

// fn distance(one: Vec2, other: Vec2) -> f32 {
//     ((one.x - other.x).powi(2) + (one.y - other.y).powi(2)).sqrt()
// }
