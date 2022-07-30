use bevy::{
    core::FixedTimestep,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use bevy_mod_picking::*;
use bevy_obj::*;
use robots_sim::{InfiniteGridBundle, InfiniteGridPlugin};

use robots_sim::elbow::*;
use robots_sim::lower_arm::*;
use robots_sim::shoulder::*;
use robots_sim::upper_arm::*;
use robots_sim::wrist::*;

const TIME_STEP: f32 = 1.0 / 60.0;
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Robots-sim".to_string(),
            mode: WindowMode::Windowed,
            // present_mode: PresentMode::Fifo,
            present_mode: PresentMode::Immediate,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(infotext_system)
        .add_system(change_text_system)
        .add_startup_system(setup)
        .add_plugins(DefaultPickingPlugins)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                // .with_system(debug_zy_plane) // Debug dunction
                .with_system(choise_object)
                .with_system(shoulder_rotate)
                .with_system(lower_arm_rotate)
                .with_system(elbow_rotate)
                .with_system(upper_arm_rotate)
                .with_system(wrist_rotate),
        )
        // .add_plugin(DebugCursorPickingPlugin) // <- Adds the green debug cursor.
        // .add_plugin(DebugEventsPickingPlugin) // <- Adds debug event logging.
        .add_plugin(ObjPlugin)
        .add_plugin(InfiniteGridPlugin)
        .add_plugin(NoCameraPlayerPlugin)
        // Default Movement Settings: sensitivity = 0.00012, speed = 12.0
        .insert_resource(MovementSettings {
            sensitivity: 0.00008,
            speed: 8.0,
        })
        .run();
}

#[derive(Component, Debug)]
#[allow(unused)]
struct MoveObject {
    move_speed: f32,
}
#[derive(Component)]
struct TextChanges;
fn infotext_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        },
        text: Text::with_section(
            "This is\ntext with\nline breaks\nin the top left",
            TextStyle {
                font: font.clone(),
                font_size: 50.0,
                color: Color::WHITE,
            },
            Default::default(),
        ),
        ..default()
    });
    commands.spawn_bundle(TextBundle {
    style: Style {
        align_self: AlignSelf::FlexEnd,
        position_type: PositionType::Absolute,
        position: Rect {
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        },
        max_size: Size {
            width: Val::Px(400.),
            height: Val::Undefined,
        },
        ..default()
    },
    text: Text::with_section(
                "This text is very long, has a limited width, is centred, is positioned in the top right and is also coloured pink.",
                    TextStyle {
                font: font.clone(),
                font_size: 50.0,
                color: Color::rgb(0.8, 0.2, 0.7),
            },
        TextAlignment {
            horizontal: HorizontalAlign::Center,
            vertical: VerticalAlign::Center,
        },
    ),
    ..default()
});
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "This text changes in the bottom right".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "\nThis text changes in the bottom right - ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: Color::RED,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: Color::ORANGE_RED,
                        },
                    },
                    TextSection {
                        value: " fps, ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: Color::YELLOW,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: Color::GREEN,
                        },
                    },
                    TextSection {
                        value: " ms/frame".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: Color::BLUE,
                        },
                    },
                ],
                alignment: Default::default(),
            },
            ..default()
        })
        .insert(TextChanges);
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            size: Size {
                width: Val::Px(200.0),
                ..default()
            },
            ..default()
        },
        text: Text::with_section(
            "This\ntext has\nline breaks and also a set width in the bottom left".to_string(),
            TextStyle {
                font,
                font_size: 50.0,
                color: Color::WHITE,
            },
            Default::default(),
        ),
        ..default()
    });
}
fn change_text_system(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<TextChanges>>,
) {
    for mut text in query.iter_mut() {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diagnostic.average() {
                fps = fps_avg;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_avg) = frame_time_diagnostic.average() {
                frame_time = frame_time_avg;
            }
        }

        text.sections[0].value = format!(
            "This text changes in the bottom right - {:.1} fps, {:.3} ms/frame",
            fps,
            frame_time * 1000.0,
        );

        text.sections[2].value = format!("{:.1}", fps);

        text.sections[4].value = format!("{:.3}", frame_time * 1000.0);
    }
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Grid and Axies
    commands.spawn_bundle(InfiniteGridBundle::default());
    // Camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(4.0, 2.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCam)
        .insert_bundle(PickingCameraBundle::default());

    // Spawn objects
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
        });

    commands
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
        });

    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("models/Gleb_Robot/elbow.obj"),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(1., 1., 1.),
                Quat::from_rotation_y(0.),
                Vec3::new(0.0, 1.55, -1.25),
            )),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(ElbowRotate {
            rotation_speed,
            can_move: false,
        });
    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("models/Gleb_Robot/upper_arm.obj"),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(1., 1., 1.),
                Quat::from_rotation_x(0.),
                Vec3::new(0.0, 2.446, 0.48),
            )),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(UpperArmRotate {
            rotation_speed,
            can_move: false,
        });
    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("models/Gleb_Robot/wrist.obj"),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(1., 1., 1.),
                Quat::from_rotation_x(0.25844246),
                Vec3::new(0.0, 2.368001, 0.79499954),
            )),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(WristRotate {
            rotation_speed,
            can_move: false,
        });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 15000.0,
            ..default()
        },
        transform: Transform::from_xyz(-5.0, 8.0, -5.0),
        ..default()
    });
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 15000.0,
            ..default()
        },
        transform: Transform::from_xyz(5.0, 8.0, 5.0),
        ..default()
    });
    // Blue background
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            // We make the dimensions negative because we want to invert the direction
            // of light the mesh diffuses (invert the normals).
            radius: -150.0,
            depth: -1.0,
            ..Default::default()
        })),
        // We make the mesh as rough as possible to avoid metallic-like reflections
        material: materials.add(StandardMaterial {
            perceptual_roughness: 1.0,
            reflectance: 0.0,
            emissive: Color::rgb(0.0, 0.05, 0.5),
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 1.0, 1.0)),
        ..Default::default()
    });

    // Debug Cube
    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube::new(0.1))),
    //         material: materials.add(StandardMaterial {
    //             perceptual_roughness: 1.0,
    //             emissive: Color::rgb(0.0, 0.05, 0.5),
    //             ..Default::default()
    //         }),
    //         transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 1.0, 1.0)),
    //         ..Default::default()
    //     })
    //     .insert(MoveObject { move_speed: 0.001 });
}

fn choise_object(
    keyboard_input: Res<Input<KeyCode>>,
    mut query1: Query<&mut ShoulderRotate>,
    mut query2: Query<&mut LowerArmRotate>,
    mut query3: Query<&mut ElbowRotate>,
    mut query4: Query<&mut UpperArmRotate>,
    mut query5: Query<&mut WristRotate>,
) {
    let mut object1 = query1.single_mut();
    let mut object2 = query2.single_mut();
    let mut object3 = query3.single_mut();
    let mut object4 = query4.single_mut();
    let mut object5 = query5.single_mut();
    object1.can_move = false;
    object2.can_move = false;
    object3.can_move = false;
    object4.can_move = false;
    object5.can_move = false;
    if keyboard_input.pressed(KeyCode::Key1) {
        object1.can_move = true;
    } else if keyboard_input.pressed(KeyCode::Key2) {
        object2.can_move = true;
    } else if keyboard_input.pressed(KeyCode::Key3) {
        object3.can_move = true;
    } else if keyboard_input.pressed(KeyCode::Key4) {
        object4.can_move = true;
    } else if keyboard_input.pressed(KeyCode::Key5) {
        object5.can_move = true;
    }
}

// Debug function
// add Component MoveObject to debug
// allows move and rotate object on one plane
// Usage:
//     Move: "I" "K" "J" "L"
//     Rotate: "U" "O"
//     Print: "Z"
#[allow(unused)]
fn debug_zy_plane(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&MoveObject, &mut Transform)>,
) {
    let (obj, mut transform) = query.single_mut();
    let movement_factor = obj.move_speed;
    let rotation_speed: f32 = f32::to_radians(1.);
    if keyboard_input.pressed(KeyCode::I) {
        transform.translation.y += movement_factor;
    }
    if keyboard_input.pressed(KeyCode::K) {
        transform.translation.y -= movement_factor;
    }
    if keyboard_input.pressed(KeyCode::J) {
        transform.translation.z -= movement_factor;
    }
    if keyboard_input.pressed(KeyCode::L) {
        transform.translation.z += movement_factor;
    }
    if keyboard_input.pressed(KeyCode::U) {
        transform.rotate(Quat::from_rotation_x(rotation_speed));
    }
    if keyboard_input.pressed(KeyCode::O) {
        transform.rotate(Quat::from_rotation_x(-rotation_speed));
    }
    if keyboard_input.pressed(KeyCode::Z) {
        dbg!(transform);
    }
}
