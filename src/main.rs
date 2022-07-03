use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_flycam::{MovementSettings, PlayerPlugin};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Robots-sim".to_string(),
            mode: WindowMode::Windowed,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(InfiniteGridPlugin)
        .add_startup_system(setup)
        .add_plugin(PlayerPlugin) // Camera
        .insert_resource(MovementSettings {
            sensitivity: 0.00008, // default: 0.00012
            speed: 8.0,           // default: 12.0
        })
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Grid and Axies
    commands.spawn_bundle(InfiniteGridBundle::default());
    // Robot
    commands.spawn_scene(asset_server.load("models/kuka_0/scene.gltf#Scene0"));
    // BLOCK Lights
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
    // END BLOCK OF LIGHT
}
