use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct WristRotate {
    pub rotation_speed: f32,
    pub can_move: bool,
}
pub fn wrist_rotate(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&WristRotate, &mut Transform)>,
) {
    let (object, mut transform) = query.single_mut();
    let mut rotation_factor = 0.0;
    if object.can_move {
        if keyboard_input.pressed(KeyCode::Left) {
            rotation_factor += object.rotation_speed;
            transform.rotate(Quat::from_rotation_y(rotation_factor));
        }
        if keyboard_input.pressed(KeyCode::Right) {
            rotation_factor -= object.rotation_speed;
            transform.rotate(Quat::from_rotation_y(rotation_factor));
        }
        if keyboard_input.pressed(KeyCode::Up) {
            rotation_factor -= object.rotation_speed;
            transform.rotate(Quat::from_rotation_x(rotation_factor));
        }
        if keyboard_input.pressed(KeyCode::Down) {
            rotation_factor += object.rotation_speed;
            transform.rotate(Quat::from_rotation_x(rotation_factor));
        }
    }
}
