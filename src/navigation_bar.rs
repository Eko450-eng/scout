use egui::Ui;

use crate::{movement_actions::move_out, types::FilesApp};

pub fn navigation_bar(ui: &mut Ui, app: &mut FilesApp) {
    let mut history: Vec<String> = vec![];
    for h in app.history.clone() {
        history.push(h.to_string_lossy().to_string())
    }

    let mut back_path = app.current_path.clone();
    back_path.pop();

    ui.horizontal_top(|ui| {
        if ui.add(egui::Button::new("<")).clicked() {
            move_out(app)
        }
        if ui.add(egui::Button::new("Settings")).clicked() {
            app.setting = true
        }
    });
}
