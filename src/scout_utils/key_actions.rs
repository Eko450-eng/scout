use egui::{Modifiers, Ui};

use crate::{
    poups::search_file_popup::reset_search,
    types::{FilesApp, Modes},
};

use super::{
    action_functions::{jump_bottom, jump_top, key_quit, rename_selected_file, search, toggle_debug, toggle_hidden, toggle_mode, toggle_preview}, file_man::{delete_file, save_to_file}, movement_actions::{move_back, move_down, move_in, move_out, move_up}
};

pub fn handle_key_action(app: &mut FilesApp, ui: &mut Ui) {
    if ui.input(|i| i.to_owned().key_pressed(egui::Key::Questionmark)) {
        // TODO: Implement show help
        // show_help()
    } else if ui.input(|i| i.to_owned().consume_key(Modifiers::CTRL, egui::Key::Q)) {
        key_quit(app, ui)
    } else if ui.input(|i| i.to_owned().consume_key(Modifiers::SHIFT, egui::Key::G)) {
        jump_bottom(app)
    } else if ui.input(|i| i.key_pressed(egui::Key::G)) {
        jump_top(app, ui)
    } else if ui.input(|i| i.key_pressed(app.keybinds.debug)) {
        toggle_debug(app)
    } else if ui.input(|i| i.key_pressed(app.keybinds.reset_search)) {
        reset_search(app)
    } else if ui.input(|i| i.key_pressed(egui::Key::I)) {
        toggle_mode(app)
    } else if ui.input(|i| i.key_pressed(app.keybinds.hide_hidden_files)) {
        toggle_hidden(app);
    } else if ui.input(|i| i.key_pressed(app.keybinds.preview)) {
        toggle_preview(app)
    } else if ui.input(|i| i.key_pressed(app.keybinds.search)) {
        search(app)
    } else if ui.input(|i| i.key_pressed(app.keybinds.create)) {
        app.app_mode = Modes::Creation
    } else if ui.input(|i| i.key_pressed(app.keybinds.delete)) {
        app.app_mode = Modes::Deletion
    } else if ui.input(|i| i.key_pressed(app.keybinds.rename)) {
        rename_selected_file(app)
    } else if ui.input(|i| i.key_pressed(app.keybinds.move_down)) {
        move_down(app)
    } else if ui.input(|i| i.key_pressed(app.keybinds.move_up)) {
        move_up(app)
    } else if ui.input(|i| i.key_pressed(app.keybinds.move_back)) {
        move_back(app)
    } else if ui.input(|i| i.key_pressed(app.keybinds.move_out)) {
        move_out(app)
    } else if ui.input(|i| (i.key_pressed(egui::Key::Enter)) || i.key_pressed(app.keybinds.move_in))
    {
        move_in(app)
    };
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
