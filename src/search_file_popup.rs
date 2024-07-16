use egui::Context;

use crate::{add_file_popup::center_popup, file_man::get_root_dir_files, types::Modes, FilesApp};

pub fn search_file_popup(
    ctx: Context,
    app: &mut FilesApp,
) -> Option<egui::InnerResponse<Option<()>>> {
    let (pos, size)=center_popup(ctx.clone());

    let window = egui::Window::new("Search")
        .default_size(size)
        .default_open(true)
        .default_pos(pos);

    match app.app_mode {
        Modes::Search => window.show(&ctx, |ui| {
            ui.text_edit_singleline(&mut app.search_string)
                .request_focus();

            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let mut path = app.selected_element.clone().path;
                path.pop();

                app.files = get_root_dir_files(
                    app.current_path.clone(),
                    app.hide_hidden_files,
                    app.search_string.clone(),
                );

                app.selected_element_index = 0;
                if app.files.first().is_some() {
                    app.selected_element = app
                        .files
                        .first()
                        .expect("Can't get the first element")
                        .clone();
                }
            } else if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                app.app_mode = Modes::Action
            }
        }),
        _ => return None,
    }
}

pub fn reset_search(app: &mut FilesApp) {
    app.files = get_root_dir_files(
        app.current_path.clone(),
        app.hide_hidden_files,
        "".to_string(),
    );
    app.selected_element_index = 0;
}
