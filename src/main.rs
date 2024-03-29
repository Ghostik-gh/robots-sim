use bevy::{
    core::FixedTimestep,
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_egui::EguiPlugin;
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use bevy_obj::*;

use robots_sim::elbow::*;
use robots_sim::lower_arm::*;
use robots_sim::robot::*;
use robots_sim::shoulder::*;
use robots_sim::side_panel::*;
use robots_sim::upper_arm::*;
use robots_sim::wrist::*;
use robots_sim::{InfiniteGridBundle, InfiniteGridPlugin};

const TIME_STEP: f32 = 1.0 / 60.0;
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Robots-sim".to_string(),
            mode: WindowMode::Windowed,
            present_mode: PresentMode::Immediate,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<UiState>()
        .add_system(ui_example)
        .add_startup_system(setup_robot)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(choise_object),
        )
        .add_plugin(ObjPlugin)
        .add_plugin(InfiniteGridPlugin)
        .add_plugin(NoCameraPlayerPlugin)
        // Default Movement Settings: sensitivity = 0.00012, speed = 12.0
        .insert_resource(MovementSettings {
            sensitivity: 0.00006,
            speed: 7.0,
        })
        .run();
}

fn setup(
    mut commands: Commands,
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
        .insert(FlyCam);
    // Light
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
    // Background
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: -150.0,
            depth: -1.0,
            ..Default::default()
        })),
        material: materials.add(StandardMaterial {
            perceptual_roughness: 1.0,
            reflectance: 0.0,
            emissive: Color::rgb(0.0, 0.05, 0.5),
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 1.0, 1.0)),
        ..Default::default()
    });
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
    if keyboard_input.pressed(KeyCode::Key1) {
        object1.can_move = true;
        object2.can_move = false;
        object3.can_move = false;
        object4.can_move = false;
        object5.can_move = false;
    } else if keyboard_input.pressed(KeyCode::Key2) {
        object1.can_move = false;
        object2.can_move = true;
        object3.can_move = false;
        object4.can_move = false;
        object5.can_move = false;
    } else if keyboard_input.pressed(KeyCode::Key3) {
        object1.can_move = false;
        object2.can_move = false;
        object3.can_move = true;
        object4.can_move = false;
        object5.can_move = false;
    } else if keyboard_input.pressed(KeyCode::Key4) {
        object1.can_move = false;
        object2.can_move = false;
        object3.can_move = false;
        object4.can_move = true;
        object5.can_move = false;
    } else if keyboard_input.pressed(KeyCode::Key5) {
        object1.can_move = false;
        object2.can_move = false;
        object3.can_move = false;
        object4.can_move = false;
        object5.can_move = true;
    }
}

// #[derive(Component, Debug)]
// #[allow(unused)]
// struct MoveObject {
//     move_speed: f32,
// }
// Debug function
// add Component MoveObject to debug
// allows move and rotate object on one plane
// Usage:
//     Move: "I" "K" "J" "L"
//     Rotate: "U" "O"
//     Print: "Z"
// #[allow(unused)]
// fn debug_zy_plane(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<(&MoveObject, &mut Transform)>,
// ) {
//     let (obj, mut transform) = query.single_mut();
//     let movement_factor = obj.move_speed;
//     let rotation_speed: f32 = f32::to_radians(1.);
//     if keyboard_input.pressed(KeyCode::I) {
//         transform.translation.y += movement_factor;
//     }
//     if keyboard_input.pressed(KeyCode::K) {
//         transform.translation.y -= movement_factor;
//     }
//     if keyboard_input.pressed(KeyCode::J) {
//         transform.translation.z -= movement_factor;
//     }
//     if keyboard_input.pressed(KeyCode::L) {
//         transform.translation.z += movement_factor;
//     }
//     if keyboard_input.pressed(KeyCode::U) {
//         transform.rotate(Quat::from_rotation_x(rotation_speed));
//     }
//     if keyboard_input.pressed(KeyCode::O) {
//         transform.rotate(Quat::from_rotation_x(-rotation_speed));
//     }
//     if keyboard_input.pressed(KeyCode::Z) {
//         dbg!(transform);
//     }
// }
