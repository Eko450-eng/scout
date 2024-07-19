use egui::Ui;

use crate::types::{FilesApp, Modes};

use super::{file_man::get_root_dir_files, utils::save_filesapp_state};


pub fn key_quit(app: &mut FilesApp, ui: &mut Ui) {
    save_filesapp_state(app);
    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
}

pub fn jump_bottom(app: &mut FilesApp) {
    app.selected_element_index = app.files.len() - 1
}

pub fn jump_top(app: &mut FilesApp, ui: &mut Ui) {
    if app.double_g && ui.input(|i| i.key_pressed(egui::Key::G)) {
        app.selected_element_index = 0;
        app.double_g = false
    } else {
        app.double_g = true
    }
}

pub fn toggle_debug(app: &mut FilesApp) {
    if app.debug {
        app.debug = false
    } else {
        app.debug = true
    }
}

pub fn toggle_hidden(app: &mut FilesApp) {
    app.hide_hidden_files = !app.hide_hidden_files;
    app.files = get_root_dir_files(
        app.current_path.clone(),
        app.hide_hidden_files,
        "".to_string(),
    );
    app.selected_element_index = 0;
}

pub fn toggle_preview(app: &mut FilesApp) {
    if app.preview {
        app.preview = false
    } else {
        app.preview = true
    }
}

pub fn search(app: &mut FilesApp) {
    app.app_mode = Modes::Search;
    app.selected_element_index = 0;
}

pub fn rename_selected_file(app: &mut FilesApp) {
    app.target = app.current_path.to_string_lossy().to_string() + &app.seperator;
    app.app_mode = Modes::Renaming
}

pub fn toggle_mode(app: &mut FilesApp) {
    match app.app_mode {
        Modes::Action => app.app_mode = Modes::Editing,
        Modes::Editing => app.app_mode = Modes::Action,
        Modes::Creation => app.app_mode = Modes::Action,
        Modes::Search => app.app_mode = Modes::Action,
        Modes::Renaming => app.app_mode = Modes::Action,
        Modes::Deletion => app.app_mode = Modes::Action,
        Modes::NonAction => app.app_mode = Modes::Action,
        Modes::Help => app.app_mode = Modes::Action,
    }
}
