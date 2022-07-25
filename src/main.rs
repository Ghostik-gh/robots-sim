use std::f32::consts::PI;

use bevy::{
    core::FixedTimestep,
    // input::keyboard::KeyboardInput,
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use bevy_mod_picking::*;
use bevy_obj::*;
use robots_sim::{InfiniteGridBundle, InfiniteGridPlugin};

const TIME_STEP: f32 = 1.0 / 60.0;
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Robots-sim".to_string(),
            mode: WindowMode::Windowed,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugins(DefaultPickingPlugins)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(base_rotate)
                .with_system(shoulder_rotate),
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
// Resource to hold the scene `instance_id` until it is loaded
// #[derive(Default)]
// struct SceneInstance(Option<InstanceId>);
// // Component that will be used to tag entities in the scene
// #[derive(Component)]
// struct EntityInMyScene;

#[derive(Component)]
struct BaseRotate {
    rotation_speed: f32,
}

#[derive(Component, Debug)]
struct ShoulderRotate {
    rotation_speed: f32,
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
    let radian: f32 = PI / 180.;
    let rotation_speed: f32 = f32::to_radians(1.);
    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(0.0, 2.0, 0.0)))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("models/Gleb_Robot/base.obj"));
            //"models/details_kuka_0/TEST.gltf#Scene0"
        })
        .insert_bundle(PickableBundle::default());

    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("models/Gleb_Robot/base.obj"),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(BaseRotate {
            rotation_speed: f32::to_radians(2.), //degrees per second
        });
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
        .insert(ShoulderRotate { rotation_speed });

    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("models/Gleb_Robot/lower_arm.obj"),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0., 0.8, 0.25)), //::from_xyz(0.0, 0.8, 0.25),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default());

    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("models/Gleb_Robot/elbow.obj"),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(1., 1., 1.),
                Quat::from_rotation_y(180. * radian),
                Vec3::new(0.0, 0.73, -1.47),
            )),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("models/Gleb_Robot/upper_arm.obj"),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.9, 1.85),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(PbrBundle {
            mesh: asset_server.load("models/Gleb_Robot/wrist.obj"),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 1., 0.32),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default());

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
}

fn base_rotate(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&BaseRotate, &mut Transform)>,
) {
    let (object, mut transform) = query.single_mut();
    let mut rotation_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += object.rotation_speed;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= object.rotation_speed;
    }

    transform.rotate(Quat::from_rotation_z(rotation_factor));
}

fn shoulder_rotate(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&ShoulderRotate, &mut Transform)>,
) {
    let (object, mut transform) = query.single_mut();
    let mut rotation_factor = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += object.rotation_speed;
        dbg!(&transform);
        dbg!(&object);
    }
    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= object.rotation_speed;
    }

    transform.rotate(Quat::from_rotation_z(rotation_factor));
}
