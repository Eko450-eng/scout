use egui::Context;

use crate::{
    scout_utils::file_man::get_root_dir_files,
    types::{FilesApp, Modes},
};

use super::add_file_popup::center_popup;

pub fn search_file_popup(
    ctx: Context,
    app: &mut FilesApp,
) -> Option<egui::InnerResponse<Option<()>>> {
    let (pos, size) = center_popup(ctx.clone());

    let window = egui::Window::new("Search")
        .default_size(size)
        .default_open(true)
        .default_pos(pos);

    match app.app_mode {
        Modes::Help => window.show(&ctx, |ui| {

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
