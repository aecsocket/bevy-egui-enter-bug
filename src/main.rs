use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            EguiPlugin,
        ))
        .init_resource::<Text>()
        .add_systems(Update, ui)
        .run();
}

#[derive(Debug, Default, Resource)]
struct Text(String);

fn ui(mut egui: EguiContexts, mut text: ResMut<Text>) {
    egui::Window::new("Window").show(egui.ctx_mut(), |ui| {
        ui.add(egui::TextEdit::singleline(&mut text.0));
    });
}
