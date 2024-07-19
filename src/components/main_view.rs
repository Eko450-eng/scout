use egui::{Align, Color32, Pos2, Rangef, Rect, ScrollArea, Ui};

use crate::{
    scout_utils::file_man::{get_content, get_root_dir_files},
    types::FilesApp,
};

pub fn main_view(ui: &mut Ui, app: &mut FilesApp) {
    ScrollArea::vertical().show(ui, |ui| {
        ui.vertical(|ui| {
            for (index, item) in app.files.clone().iter().enumerate() {
                let color = if app.selected_element_index == index {
                    Color32::from_rgba_unmultiplied(0, 10, 255, 50)
                } else if app.multiselect.contains(&index) {
                    Color32::from_rgba_unmultiplied(255, 255, 0, 50)
                } else {
                    Color32::from_rgba_premultiplied(0, 0, 0, 50)
                };

                let i = ui.label(&item.name);
                let mut rect = i.rect;

                let min: Pos2 = Pos2 {
                    x: rect.x_range().min - 1.0,
                    y: rect.y_range().min - 1.0,
                };
                let max = Pos2 {
                    x: rect.x_range().max + 1.0,
                    y: rect.y_range().max + 1.0,
                };
                rect = Rect { min, max };

                ui.painter().rect_filled(rect, 5.0, color);

                if app.selected_element_index == index {
                    ui.scroll_to_rect(i.rect, Some(Align::Center));
                }
                if i.double_clicked() {
                    app.history.push(item.clone().path);
                    app.files = get_root_dir_files(
                        item.clone().path,
                        app.hide_hidden_files,
                        app.search_string.clone(),
                    );
                } else if i.clicked() {
                    app.selected_element = item.clone();
                    app.selected_element_index = index;
                    app.content = get_content(item.clone().path)
                }
            }
        });
    });
}
