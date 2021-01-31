use bevy::{
    prelude::*,
    window::CursorMoved,
    input::mouse::{MouseMotion, MouseWheel},
    render::camera::Camera
};

#[derive(Default)]
pub struct MouseState {
    cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
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

pub (super) fn mouse_drag(
    mut state: Local<MouseState>,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut camera_query: Query<(&Camera, &mut Transform)>
) {
     if mouse_button_input.pressed(MouseButton::Right)  {
        for (_, ref mut transform) in camera_query.iter_mut() {
            for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
                let height = transform.translation.z;
                let drag_vec = transform.rotation * Vec3::new(-event.delta.x, event.delta.y, 0f32);
                let magnitude = 0.1;


                transform.translation += drag_vec * magnitude;
                transform.translation.z = height;
            }
        }
    }
}

// pub (super) fn camera_rotation(
//     mut state: Local<MouseState>,
//     mouse_button_input: Res<Input<MouseButton>>,
//     mouse_motion_events: Res<Events<MouseMotion>>,
//     mut camera_query: Query<(&Camera, &mut Transform)>
// ) {
//     if mouse_button_input.pressed(MouseButton::Left)  {
//         for (_, mut transform) in camera_query.iter_mut() {
//             for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
//                 let magnitude = 0.004;
//                 let drag_vec = event.delta * magnitude;
//                 let camera_drag = Quat::from_rotation_y(drag_vec.x) * Quat::from_rotation_x(drag_vec.y);

//                 transform.rotation *= camera_drag;
//             }
//         }
//     }
// }

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
