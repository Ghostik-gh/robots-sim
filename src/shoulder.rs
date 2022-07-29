use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct ShoulderRotate {
    pub rotation_speed: f32,
    pub can_move: bool,
}
pub fn shoulder_rotate(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&ShoulderRotate, &mut Transform)>,
) {
    let (object, mut transform) = query.single_mut();
    let mut rotation_factor = 0.0;
    if object.can_move {
        if keyboard_input.pressed(KeyCode::Left) {
            rotation_factor += object.rotation_speed;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            rotation_factor -= object.rotation_speed;
        }
        transform.rotate(Quat::from_rotation_y(rotation_factor));
    }
}
