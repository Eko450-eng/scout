use egui::Ui;

use crate::{movement_actions::move_out, types::FilesApp};

pub fn navigation_bar(ui: &mut Ui, app: &mut FilesApp) {
    let mut history: Vec<String> = vec![];
    for h in app.history.clone() {
        history.push(h.to_string_lossy().to_string())
    }

    let mut back_path = app.current_path.clone();
    back_path.pop();

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label("Back Path: ");
            ui.label(back_path.to_string_lossy().to_string())
        });

        ui.horizontal(|ui| {
            ui.label("Last Path: ");
            ui.label(app.last_path.to_string_lossy().to_string())
        });

        ui.horizontal(|ui| {
            ui.label("Current Path: ");
            ui.label(app.current_path.to_string_lossy().to_string());
        });

        if ui.add(egui::Button::new("<")).clicked() {
            move_out(app)
        }
    });
}
