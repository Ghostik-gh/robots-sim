use bevy::prelude::*;

use crate::elbow::ElbowRotate;

#[derive(Component, Debug)]
pub struct LowerArmRotate {
    pub rotation_speed: f32,
    pub can_move: bool,
}

pub fn lower_arm_rotate(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&LowerArmRotate, &mut Transform), Without<ElbowRotate>>,
    mut query_2: Query<(&ElbowRotate, &mut Transform), Without<LowerArmRotate>>,
) {
    let (object, mut transform) = query.single_mut();
    // =======================================================================================
    // Need create some point for sync movement
    let (_elbow, mut _transform_elbow) = query_2.single_mut();
    // =======================================================================================
    let mut rotation_factor = 0.0;
    let _radius = 1.6770509831248;
    if object.can_move {
        if keyboard_input.pressed(KeyCode::Left) {
            if transform.rotation.x < 0.7 {
                rotation_factor += object.rotation_speed;
                transform.rotate(Quat::from_rotation_x(rotation_factor));
                // transform_elbow.translation.z =
                //     radius * f32::sin(transform.rotation.x) + transform.translation.z;
                // transform_elbow.translation.y =
                //     radius * f32::cos(transform.rotation.x) + transform.translation.y;
            }
            //    0.0,    1.55,         -1.25
            //    0.0,    0.5699963,    -1.3780084,
        }
        if keyboard_input.pressed(KeyCode::Right) {
            if transform.rotation.x > -0.3 {
                rotation_factor -= object.rotation_speed;
                transform.rotate(Quat::from_rotation_x(rotation_factor));
                // transform_elbow.translation.z =
                //     radius * f32::sin(transform.rotation.x) + transform.translation.z;
                // transform_elbow.translation.y =
                //     radius * f32::cos(transform.rotation.x) + transform.translation.y;
            }
        }
    }
    // transform.rotate(Quat::from_rotation_x(rotation_factor));
}
