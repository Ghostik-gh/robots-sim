use std::f32::consts::PI;

use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
    scene::InstanceId,
    window::{PresentMode, WindowMode},
};
use bevy_flycam::{MovementSettings, PlayerPlugin};
use robots_sim::{InfiniteGridBundle, InfiniteGridPlugin};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Robots-sim".to_string(),
            mode: WindowMode::Windowed,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<SceneInstance>()
        .add_plugin(InfiniteGridPlugin)
        .add_startup_system(setup)
        .add_system(scene_update)
        // .add_system(print_keyboard_event_system)
        .add_plugin(PlayerPlugin) // Camera
        .insert_resource(MovementSettings {
            sensitivity: 0.00008, // default: 0.00012
            speed: 8.0,           // default: 12.0
        })
        .add_system(move_scene_entities)
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
    mut scene_spawner: ResMut<SceneSpawner>,
    mut scene_instance: ResMut<SceneInstance>,
) {
    // let path_to_robo = "models/Black_Honey/scene.gltf#Scene0";
    let path_to_robo = "models/details_kuka_0/TEST.gltf#Scene0";
    // Grid and Axies
    commands.spawn_bundle(InfiniteGridBundle::default());
    // 2 Robots
    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(-2.0, 0.0, -2.0)))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("models/details_kuka_0/TEST.gltf#Scene0"));
        });
    let instance_id = scene_spawner.spawn(asset_server.load(path_to_robo));
    scene_instance.0 = Some(instance_id);
    let instance_id = scene_spawner.spawn(asset_server.load(path_to_robo));
    scene_instance.0 = Some(instance_id);
    // Lights
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

// This system Read keyboard event and print them
// fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
//     for event in keyboard_input_events.iter() {
//         info!("{:?}", event);
//     }
// }

// This system will wait for the scene to be ready, and then tag entities from
// the scene with `EntityInMyScene`. All entities from the second scene will be
// tagged
fn scene_update(
    mut commands: Commands,
    scene_spawner: Res<SceneSpawner>,
    scene_instance: Res<SceneInstance>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Some(instance_id) = scene_instance.0 {
            if let Some(entity_iter) = scene_spawner.iter_instance_entities(instance_id) {
                entity_iter.for_each(|entity| {
                    commands.entity(entity).insert(EntityInMyScene);
                    println!("Entity: {:?}", entity);
                });
                *done = true;
            }
        }
    }
}

// This system will move all entities with component `EntityInMyScene`, so all
// entities from the second scene
fn move_scene_entities(
    // time: Res<Time>,
    mut scene_entities: Query<&mut Transform, With<EntityInMyScene>>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    let angle = PI / 180.;
    let mut current_angle = 0.;
    for event in keyboard_input_events.iter() {
        if event.scan_code == 16 && event.state == ElementState::Pressed {
            current_angle -= angle;
            ///////////////////////
        } else if event.scan_code == 18 && event.state == ElementState::Pressed {
            current_angle += angle;
            //////////////////////
        }
        for mut transform in scene_entities.iter_mut() {
            let rotation_speed = Quat::from_rotation_z(current_angle);
            transform.rotate(rotation_speed);
        }
        // info!("{:?}", event);
    }

    // let mut x = 0.;
    // for mut transform in scene_entities.iter_mut() {
    //     x += time.seconds_since_startup().cos() as f32 / 600.;
    //     transform.translation = Vec3::new(x, 0., 0.);
    // }

    // let mut direction = 1.;
    // let mut scale = 1.;
    // for mut transform in scene_entities.iter_mut() {
    //     transform.translation = Vec3::new(
    //         scale * direction * time.seconds_since_startup().sin() as f32 / 60.,
    //         0.,
    //         time.seconds_since_startup().cos() as f32 / 20.,
    //     );
    //     direction *= -1.;
    //     scale += 0.1;
    // }
}
