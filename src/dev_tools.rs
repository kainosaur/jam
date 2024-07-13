//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::{
    game::{movement::Movement, spawn::player::Player},
    screen::Screen,
};
use bevy::{dev_tools::states::log_transitions, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app
        // Egui plugin
        .add_plugins(EguiPlugin)
        .add_systems(Update, log_transitions::<Screen>)
        .add_systems(Update, (change_state_menu, entity_view_menu));
}

fn change_state_menu(mut contexts: EguiContexts, mut set_state: ResMut<NextState<Screen>>) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Change Game State").show(ctx, |ui| {
        if ui.button("Splash").clicked() {
            set_state.set(Screen::Splash);
        }
        if ui.button("Title").clicked() {
            set_state.set(Screen::Title);
        }
        if ui.button("Credits").clicked() {
            set_state.set(Screen::Credits);
        }
        if ui.button("Playing").clicked() {
            set_state.set(Screen::Playing);
        }
    });
}

fn entity_view_menu(
    mut contexts: EguiContexts,
    mut player_query: Query<(&mut Transform, &mut Movement), With<Player>>,
    all_entities: Query<Entity>,
) {
    let ctx = contexts.ctx_mut();
    // Spawn all entities in the game, see info about each one
    // Organize by entity type

    egui::Window::new("Entity View Menu").show(ctx, |ui| {
        ui.heading("Player Entity");
        for (mut transform, mut movement) in &mut player_query {
            ui.collapsing("PlayerEntity", |ui| {
                ui.heading("Position");
                ui.label("x");
                ui.add(egui::DragValue::new(&mut transform.translation.x));
                ui.label("y");
                ui.add(egui::DragValue::new(&mut transform.translation.y));
                ui.label("z");
                ui.add(egui::DragValue::new(&mut transform.translation.z));

                ui.heading("Scale");
                ui.label("x");
                ui.add(egui::DragValue::new(&mut transform.scale.x));
                ui.label("y");
                ui.add(egui::DragValue::new(&mut transform.scale.y));
                ui.label("z");
                ui.add(egui::DragValue::new(&mut transform.scale.z));

                ui.heading("Movement Speed");
                ui.add(egui::DragValue::new(&mut movement.speed));
            });
        }

        ui.heading("All Entities");
        for entity in &all_entities {
            ui.collapsing(format!("Entity: {:?}", entity.index()), |ui| {
                ui.label("TODO: Put info here")
            });
        }
        //I can't figure out how to display all entities with all its information
    });
}
