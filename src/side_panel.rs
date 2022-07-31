use crate::{
    elbow::ElbowRotate, lower_arm::LowerArmRotate, shoulder::ShoulderRotate,
    upper_arm::UpperArmRotate, wrist::WristRotate,
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

#[derive(Default)]
pub struct UiState {
    value1: f32,
    prev_value1: f32,
    value2: f32,
    prev_value2: f32,
    value3: f32,
    prev_value3: f32,
    value4: f32,
    prev_value4: f32,
    value5: f32,
    prev_value5: f32,
}

pub fn ui_example(
    mut query1: Query<(&ShoulderRotate, &mut Transform)>,
    mut query2: Query<(&LowerArmRotate, &mut Transform), Without<ShoulderRotate>>,
    mut query3: Query<
        (&ElbowRotate, &mut Transform),
        (Without<LowerArmRotate>, Without<ShoulderRotate>),
    >,
    mut query4: Query<
        (&UpperArmRotate, &mut Transform),
        (
            Without<LowerArmRotate>,
            Without<ShoulderRotate>,
            Without<ElbowRotate>,
        ),
    >,
    mut query5: Query<
        (&WristRotate, &mut Transform),
        (
            Without<LowerArmRotate>,
            Without<ShoulderRotate>,
            Without<ElbowRotate>,
            Without<UpperArmRotate>,
        ),
    >,
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
) {
    let (_, mut transform1) = query1.single_mut();
    let (_, mut transform2) = query2.single_mut();
    let (_, mut transform3) = query3.single_mut();
    let (_, mut transform4) = query4.single_mut();
    let (_, mut transform5) = query5.single_mut();

    egui::SidePanel::left("side_panel")
        .default_width(300.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Rotating");

            ui.horizontal(|ui| {
                ui.label("Shoulder: ");
            });
            ui_state.prev_value1 = ui_state.value1;
            ui.add(egui::Slider::new(&mut ui_state.value1, -180.0..=180.0));
            transform1.rotate(Quat::from_rotation_y(
                (ui_state.value1 - ui_state.prev_value1).to_radians(),
            ));

            ui.horizontal(|ui| {
                ui.label("Lower Arm: ");
            });
            ui_state.prev_value2 = ui_state.value2;
            ui.add(egui::Slider::new(&mut ui_state.value2, -40.0..=90.0));
            transform2.rotate(Quat::from_rotation_x(
                (ui_state.value2 - ui_state.prev_value2).to_radians(),
            ));

            ui.horizontal(|ui| {
                ui.label("Elbow: ");
            });
            ui_state.prev_value3 = ui_state.value3;
            ui.add(egui::Slider::new(&mut ui_state.value3, -55.0..=10.0));
            transform3.rotate(Quat::from_rotation_x(
                (ui_state.value3 - ui_state.prev_value3).to_radians(),
            ));

            ui.horizontal(|ui| {
                ui.label("Upper Arm: ");
            });
            ui_state.prev_value4 = ui_state.value4;
            ui.add(egui::Slider::new(&mut ui_state.value4, -20.0..=40.0));
            transform4.rotate(Quat::from_rotation_x(
                (ui_state.value4 - ui_state.prev_value4).to_radians(),
            ));

            ui.horizontal(|ui| {
                ui.label("Wrist: ");
            });
            ui_state.prev_value5 = ui_state.value5;
            ui.add(egui::Slider::new(&mut ui_state.value5, -180.0..=180.0));
            transform5.rotate(Quat::from_rotation_z(
                (ui_state.value5 - ui_state.prev_value5).to_radians(),
            ));
        });
}
