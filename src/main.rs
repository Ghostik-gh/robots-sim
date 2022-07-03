use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_flycam::{MovementSettings, PlayerPlugin};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Robots-sim".to_string(),
            mode: WindowMode::Windowed,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00008, // default: 0.00012
            speed: 8.0,           // default: 12.0
        })
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_scene(asset_server.load("models/kuka_0/scene.gltf#Scene0"));
    // commands.spawn_bundle(PerspectiveCameraBundle {
    //     transform: Transform::from_xyz(2.0, 2.0, 2.0).looking_at(Vec3::new(0.0, 0.7, 0.0), Vec3::Y),
    //     ..Default::default()
    // });
    const HALF_SIZE: f32 = 1.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}
