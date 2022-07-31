use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct LowerArmRotate {
    pub rotation_speed: f32,
    pub can_move: bool,
}

pub fn lower_arm_rotate(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&LowerArmRotate, &mut Transform)>,
) {
    let (object, mut transform) = query.single_mut();
    let mut rotation_factor = 0.0;
    if object.can_move {
        if keyboard_input.pressed(KeyCode::Left) {
            if transform.rotation.x < 0.7 {
                rotation_factor += object.rotation_speed;
                transform.rotate(Quat::from_rotation_x(rotation_factor));
            }
        }
        if keyboard_input.pressed(KeyCode::Right) {
            if transform.rotation.x > -0.3 {
                rotation_factor -= object.rotation_speed;
                transform.rotate(Quat::from_rotation_x(rotation_factor));
            }
        }
    }
}
