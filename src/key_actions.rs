use std::{fs, path::PathBuf};

use egui::{Modifiers, Ui};
use serde_json::json;

use crate::{
    file_man::{delete_file, get_root_dir_files, save_to_file},
    movement_actions::{move_back, move_down, move_in, move_out, move_up},
    search_file_popup::reset_search,
    types::{FilesApp, Modes},
};

pub fn read_filesapp_state(app: &mut FilesApp) {
    let config = PathBuf::from("/home/eko/.config/scout/config.json");
    match fs::metadata(config.clone()) {
        Ok(_) => {
            let config_content = fs::read_to_string(config);
            println!("Config was read: {}", config_content.unwrap());
        }
        Err(_) => (),
    }
}

pub fn save_filesapp_state(app: &mut FilesApp) {
    let config_path = PathBuf::from("/home/eko/.config/scout");
    match fs::metadata(config_path.clone()) {
        Ok(_) => (),
        Err(_) => match fs::create_dir(config_path) {
            Ok(_) => {}
            Err(e) => {
                println!("Can't create confg dir: {:?}", e)
            }
        },
    }

    let target_dir = PathBuf::from("/home/eko/.config/scout/config.json");

    let json_data = json!({
            "preview": app.preview,
            "keybinds": app.keybinds
    });

    match fs::write(target_dir, json_data.to_string()) {
        Ok(_) => {}
        Err(e) => {
            println!("Could not Save app State: {:?}", e)
        }
    }
}

pub fn handle_key_action(app: &mut FilesApp, ui: &mut Ui) {
    if ui.input(|i| i.to_owned().consume_key(Modifiers::CTRL, egui::Key::Q)) {
        save_filesapp_state(app);
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
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
        Modes::Setting => app.app_mode = Modes::Action,
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
