use egui::{Align, ScrollArea, Ui};

use crate::{
    file_man::{get_content, get_root_dir_files},
    types::FilesApp,
};

pub fn main_view(ui: &mut Ui, app: &mut FilesApp) {
    ScrollArea::vertical().show(ui, |ui| {
        ui.vertical(|ui| {
            for (index, item) in app.files.clone().iter().enumerate() {
                let i = ui.selectable_label(index == app.selected_element_index, &item.name);
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
