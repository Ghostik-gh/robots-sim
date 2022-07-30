use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

use crate::{
    elbow::ElbowRotate, lower_arm::LowerArmRotate, shoulder::ShoulderRotate,
    upper_arm::UpperArmRotate, wrist::WristRotate,
};

pub fn setup_robot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rotation_speed: f32 = f32::to_radians(1.);
    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("models/Gleb_Robot/base.obj"),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("models/Gleb_Robot/shoulder.obj"),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(1., 1., 1.),
                Quat::from_rotation_x(0.),
                Vec3::new(0.0, 0., 0.),
            )),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(ShoulderRotate {
            rotation_speed,
            can_move: false,
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    mesh: asset_server.load("models/Gleb_Robot/lower_arm.obj"),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                        Vec3::new(1., 1., 1.),
                        Quat::from_rotation_y(0.),
                        Vec3::new(0., 0.8, 0.25),
                    )),
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default())
                .insert(LowerArmRotate {
                    rotation_speed,
                    can_move: false,
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(PbrBundle {
                            mesh: asset_server.load("models/Gleb_Robot/elbow.obj"),
                            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                            transform: Transform::from_matrix(
                                Mat4::from_scale_rotation_translation(
                                    Vec3::new(1., 1., 1.),
                                    Quat::from_rotation_y(0.),
                                    Vec3::new(0.0, 0.7379941, -1.5010117),
                                ),
                            ),
                            ..Default::default()
                        })
                        .insert_bundle(PickableBundle::default())
                        .insert(ElbowRotate {
                            rotation_speed,
                            can_move: false,
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(PbrBundle {
                                    mesh: asset_server.load("models/Gleb_Robot/upper_arm.obj"),
                                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                                    transform: Transform::from_matrix(
                                        Mat4::from_scale_rotation_translation(
                                            Vec3::new(1., 1., 1.),
                                            Quat::from_rotation_x(0.),
                                            Vec3::new(0.0, 0.9099869, 1.7750295),
                                        ),
                                    ),
                                    ..Default::default()
                                })
                                .insert_bundle(PickableBundle::default())
                                .insert(UpperArmRotate {
                                    rotation_speed,
                                    can_move: false,
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(PbrBundle {
                                            mesh: asset_server.load("models/Gleb_Robot/wrist.obj"),
                                            material: materials
                                                .add(Color::rgb(0.8, 0.7, 0.6).into()),
                                            transform: Transform::from_matrix(
                                                Mat4::from_scale_rotation_translation(
                                                    Vec3::new(1., 1., 1.),
                                                    Quat::from_rotation_x(0.25844246),
                                                    Vec3::new(0.0, -0.09199781, 0.33600545),
                                                ),
                                            ),
                                            ..Default::default()
                                        })
                                        .insert_bundle(PickableBundle::default())
                                        .insert(WristRotate {
                                            rotation_speed,
                                            can_move: false,
                                        });
                                });
                        });
                });
        });
}
