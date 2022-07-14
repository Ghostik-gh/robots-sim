use std::{convert::identity, f32::consts::PI};

use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
    scene::InstanceId,
    window::{PresentMode, WindowMode},
};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use bevy_mod_picking::*;
use bevy_obj::*;
use robots_sim::{InfiniteGridBundle, InfiniteGridPlugin};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Robots-sim".to_string(),
            mode: WindowMode::Windowed,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugin(DebugCursorPickingPlugin) // <- Adds the green debug cursor.
        .add_plugin(DebugEventsPickingPlugin) // <- Adds debug event logging.
        .add_plugins(DefaultPlugins)
        .add_plugin(ObjPlugin)
        .init_resource::<SceneInstance>()
        .add_plugin(InfiniteGridPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_startup_system(setup)
        .add_system(selector_obj)
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00008, // default: 0.00012
            speed: 8.0,           // default: 12.0
        })
        // .add_system(move_scene_entities)
        .run();
}
// Resource to hold the scene `instance_id` until it is loaded
#[derive(Default)]
struct SceneInstance(Option<InstanceId>);

// Component that will be used to tag entities in the scene
#[derive(Component)]
struct EntityInMyScene;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut scene_spawner: ResMut<SceneSpawner>,
    // mut scene_instance: ResMut<SceneInstance>,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let path_to_robo = "models/Black_Honey/scene.gltf#Scene0";
    // let path_to_robo = "models/details_kuka_0/TEST.gltf#Scene0";

    // Grid and Axies
    commands.spawn_bundle(InfiniteGridBundle::default());

    // commands
    //     .spawn_bundle(Camera3dBundle {
    //         transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //         ..Default::default()
    //     })
    //     .insert_bundle(PickingCameraBundle::default());

    // Camera
    let camera = PerspectiveCameraBundle {
        transform: Transform::from_xyz(4.0, 2.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };
    commands
        .spawn_bundle(camera)
        .insert(FlyCam)
        .insert_bundle(PickingCameraBundle::default());

    // Spawn objects
    // commands
    //     .spawn_bundle(TransformBundle::from(Transform::from_xyz(0.0, 2.0, 0.0)))
    //     .with_children(|parent| {
    //         parent.spawn_scene(asset_server.load("models/details_kuka_0/TEST.gltf#Scene0"));
    //     })
    //     .insert_bundle(PickableBundle::default());
    let radian = PI / 180.;
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
        .insert_bundle(PickableBundle::default());

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

    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //         transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //         ..Default::default()
    //     })
    //     .insert_bundle(PickableBundle::default());

    // let instance_id_0 = scene_spawner.spawn(asset_server.load("models/details_kuka_0/0.gltf#Scene0"));
    // scene_instance.0 = Some(instance_id_0);

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
}

fn selector_obj(
    // query_changed: Query<&Interaction, (Changed<Interaction>, Without<NoDeselect>)>,
    selection_query: Query<
        (Entity, &Selection, ChangeTrackers<Selection>),
        (Changed<Selection>, With<PickableMesh>),
    >,
    mut picking_events: EventWriter<PickingEvent>,
) {
    // let mut new_selection = false;
    // for interaction in query_changed.iter() {
    //     if *interaction == Interaction::Clicked {
    //         new_selection = true;
    //     }
    // }
    // if new_selection {}

    for (entity, selection, selection_change) in selection_query.iter() {
        if selection_change.is_added() {
            continue; // Avoid a false change detection when a component is added.
        }
        if selection.selected() {
            println!("SELECTED {:?}", entity);
        } else {
            picking_events.send(PickingEvent::Selection(SelectionEvent::JustDeselected(
                entity,
            )));
        }
    }
}

// This system will wait for the scene to be ready, and then tag entities from
// the scene with `EntityInMyScene`. All entities from the second scene will be
// tagged
// fn scene_update(
//     mut commands: Commands,
//     scene_spawner: Res<SceneSpawner>,
//     scene_instance: Res<SceneInstance>,
//     mut done: Local<bool>,
// ) {
//     if !*done {
//         if let Some(instance_id) = scene_instance.0 {
//             if let Some(entity_iter) = scene_spawner.iter_instance_entities(instance_id) {
//                 entity_iter.for_each(|entity| {
//                     commands.entity(entity).insert(EntityInMyScene);
//                     println!("Entity: {:?}", entity);
//                 });
//                 *done = true;
//             }
//         }
//     }
// }

// This system will move all entities with component `EntityInMyScene`, so all
// entities from the second scene
// fn move_scene_entities(
//     // time: Res<Time>,
//     mut scene_entities: Query<&mut Transform, With<EntityInMyScene>>,
//     mut keyboard_input_events: EventReader<KeyboardInput>,
// ) {
//     let angle = PI / 180.;
//     let mut current_angle = 0.;
//     for event in keyboard_input_events.iter() {
//         if event.scan_code == 16 && event.state == ElementState::Pressed {
//             current_angle -= angle;
//             ///////////////////////
//         } else if event.scan_code == 18 && event.state == ElementState::Pressed {
//             current_angle += angle;
//             //////////////////////
//         }
//         for mut transform in scene_entities.iter_mut() {
//             let rotation_speed = Quat::from_rotation_x(current_angle);
//             transform.rotate(rotation_speed);
//         }
//     }
// }

// This system Read keyboard event and print them
// fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
//     for event in keyboard_input_events.iter() {
//         info!("{:?}", event);
//     }
// }
