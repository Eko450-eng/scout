use egui::Ui;

use crate::{
    file_man::{get_root_dir_files, rename_file},
    movement_actions::{move_back, move_down, move_in, move_out, move_up},
    search_file_popup::reset_search,
    types::{FilesApp, ItemElement, Modes},
};

pub fn handle_key_action(app: &mut FilesApp, ui: &mut Ui) {
    if ui.input(|i| i.key_pressed(app.keybinds.reset_search)) {
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
        // Rename file
    } else if ui.input(|i| i.key_pressed(app.keybinds.rename)) {
        action_rename(app.selected_element.clone())
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
        move_in(app)
    };
}

pub fn action_rename(selected_element: ItemElement) {
    if false == true {
        rename_file(selected_element.path.clone(), "test".to_string())
    } else {
        println!("This feature is currently disabled")
    }
}

pub fn toggle_mode(app: &mut FilesApp) {
    match app.app_mode {
        Modes::Action => app.app_mode = Modes::Editing,
        Modes::Editing => app.app_mode = Modes::Action,
        Modes::Creation => app.app_mode = Modes::Action,
        Modes::Search => app.app_mode = Modes::Action,
    }
}

pub fn handle_key_search(app: &mut FilesApp, ui: &mut Ui) {
    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        app.app_mode = Modes::Action
    }
}

pub fn handle_key_creation(app: &mut FilesApp, ui: &mut Ui) {
    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        app.app_mode = Modes::Action
    }
}
