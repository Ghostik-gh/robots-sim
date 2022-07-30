use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use crate::elbow::ElbowRotate;
use crate::lower_arm::LowerArmRotate;
use crate::shoulder::ShoulderRotate;
use crate::upper_arm::UpperArmRotate;
use crate::wrist::WristRotate;

#[derive(Component)]
pub struct TextChanges;

#[derive(Component)]
pub struct ComponentChanges;

pub fn infotext_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_size = 30.;
    let color = Color::rgb(0.8, 0.2, 0.7);

    let text_style = TextStyle {
        font: font.clone(),
        font_size,
        color,
    };
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
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
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Shoulder: ".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "\nLower Arm: ".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "\nElbow: ".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "\nUpper Arm: ".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "\nWrist: ".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: text_style.clone(),
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(ComponentChanges);
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
                        value: "FPS: ".to_string(),
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
                ],
                alignment: Default::default(),
            },
            ..default()
        })
        .insert(TextChanges);
}

pub fn fps_system(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<TextChanges>>,
) {
    for mut text in query.iter_mut() {
        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_avg) = frame_time_diagnostic.average() {
                frame_time = frame_time_avg;
            }
        }
        text.sections[1].value = format!("{:.3}", frame_time * 10000.0);
    }
}

pub fn change_text_system(
    mut query: Query<&mut Text, With<ComponentChanges>>,
    query_shoulder: Query<(&ShoulderRotate, &Transform)>,
    query_lower_arm: Query<(&LowerArmRotate, &Transform)>,
    query_elbow: Query<(&ElbowRotate, &Transform)>,
    query_upper_arm: Query<(&UpperArmRotate, &Transform)>,
    query_wrist: Query<(&WristRotate, &Transform)>,
) {
    let (_shoulder, shoulder_transform) = query_shoulder.single();
    let (_lower_arm, lower_arm_transform) = query_lower_arm.single();
    let (_elbow, elbow_transform) = query_elbow.single();
    let (_upper_arm, upper_arm_transform) = query_upper_arm.single();
    let (_wrist, wrist_transform) = query_wrist.single();
    for mut text in query.iter_mut() {
        text.sections[1].value = shoulder_transform.rotation.y.to_degrees().to_string();
        text.sections[3].value = lower_arm_transform.rotation.x.to_degrees().to_string();
        text.sections[5].value = elbow_transform.rotation.x.to_degrees().to_string();
        text.sections[7].value = upper_arm_transform.rotation.x.to_degrees().to_string();
        text.sections[9].value = wrist_transform.rotation.z.to_degrees().to_string();
    }
    // dbg!(shoulder_transform);
}
