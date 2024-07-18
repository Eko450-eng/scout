use egui::{Modifiers, Ui};

use crate::{
    file_man::{delete_file, get_root_dir_files, save_to_file},
    movement_actions::{move_back, move_down, move_in, move_out, move_up},
    search_file_popup::reset_search,
    types::{FilesApp, Modes}, utils::save_filesapp_state,
};


pub fn handle_key_action(app: &mut FilesApp, ui: &mut Ui) {
    if ui.input(|i| i.to_owned().consume_key(Modifiers::CTRL, egui::Key::Q)) {
        save_filesapp_state(app);
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    } else if ui.input(|i| i.to_owned().consume_key(Modifiers::SHIFT, egui::Key::G)) {
        app.selected_element_index = app.files.len() - 1
    } else if ui.input(|i| i.key_pressed(egui::Key::G)) {
        if app.double_g && ui.input(|i| i.key_pressed(egui::Key::G)) {
            app.selected_element_index = 0;
            app.double_g = false
        } else {
            app.double_g = true
        }
    } else if ui.input(|i| i.key_pressed(app.keybinds.debug)) {
        if app.debug {
            app.debug = false
        } else {
            app.debug = true
        }
    } else if ui.input(|i| i.key_pressed(app.keybinds.reset_search)) {
        reset_search(app)
    } else if ui.input(|i| i.key_pressed(egui::Key::I)) {
        toggle_mode(app)
    } else if ui.input(|i| i.key_pressed(app.keybinds.hide_hidden_files)) {
        app.hide_hidden_files = !app.hide_hidden_files;
        app.files = get_root_dir_files(
            app.current_path.clone(),
            app.hide_hidden_files,
            "".to_string(),
        );
        app.selected_element_index = 0;
    } else if ui.input(|i| i.key_pressed(app.keybinds.preview)) {
        if app.preview {
            app.preview = false
        } else {
            app.preview = true
        }
    // Search
    } else if ui.input(|i| i.key_pressed(app.keybinds.search)) {
        app.app_mode = Modes::Search;
        app.selected_element_index = 0;
    // Create File
    } else if ui.input(|i| i.key_pressed(app.keybinds.create)) {
        app.app_mode = Modes::Creation
    // Delete File
    } else if ui.input(|i| i.key_pressed(app.keybinds.delete)) {
        app.app_mode = Modes::Deletion
        // Rename file
    } else if ui.input(|i| i.key_pressed(app.keybinds.rename)) {
        app.target = app.current_path.to_string_lossy().to_string() + "/";
        app.app_mode = Modes::Renaming
    // Move Down
    } else if ui.input(|i| {
        i.key_pressed(app.keybinds.move_down) && app.selected_element_index < app.files.len() - 1
    }) {
        move_down(app)
    // Move up
    } else if ui.input(|i| i.key_pressed(app.keybinds.move_up)) && app.selected_element_index > 0 {
        move_up(app)
    // Move out
    } else if ui.input(|i| i.key_pressed(app.keybinds.move_back)) {
        move_back(app)
    } else if ui.input(|i| i.key_pressed(app.keybinds.move_out)) {
        move_out(app)
    // Move in
    } else if ui.input(|i| (i.key_pressed(egui::Key::Enter)) || i.key_pressed(app.keybinds.move_in))
    {
        if !app.empty {
            move_in(app)
        }
    };
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
    }
}

pub fn handle_key_search(app: &mut FilesApp, ui: &mut Ui) {
    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        app.app_mode = Modes::Action
    } else if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        app.app_mode = Modes::Action
    }
}

pub fn handle_key_delete(app: &mut FilesApp, ui: &mut Ui) {
    if ui.input(|i| i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::N)) {
        app.app_mode = Modes::Action
    } else if ui.input(|i| i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Y)) {
        delete_file(app);
        app.app_mode = Modes::Action
    }
}

pub fn handle_key_creation(app: &mut FilesApp, ui: &mut Ui) {
    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        app.app_mode = Modes::Action
    }
}

pub fn handle_key_editing(app: &mut FilesApp, ui: &mut Ui) {
    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        app.app_mode = Modes::Action
    } else if ui.input(|i| i.to_owned().consume_key(Modifiers::CTRL, egui::Key::S)) {
        save_to_file(app);
        app.app_mode = Modes::Action
    }
}
