use std::path::PathBuf;

use crate::file_man::{get_root_dir_files, FileContent};

pub struct KeyBinds {
    pub debug: egui::Key,
    pub move_back: egui::Key,
    pub reset_search: egui::Key,
    pub search: egui::Key,
    pub hide_hidden_files: egui::Key,
    pub preview: egui::Key,
    pub create: egui::Key,
    pub delete: egui::Key,
    pub rename: egui::Key,
    pub move_in: egui::Key,
    pub move_out: egui::Key,
    pub move_up: egui::Key,
    pub move_down: egui::Key,
}

pub enum Modes {
    Action,
    Editing,
    Creation,
    Search,
}

#[derive(Clone, Default)]
pub struct ItemElement {
    pub name: String,
    pub path: PathBuf,
    pub dir: bool,
}

// FilesApp
pub struct FilesApp {
    pub selected_element_index: usize,
    pub selected_element: ItemElement,

    pub files: Vec<ItemElement>,
    pub preview: bool,

    pub new_file_name: String,
    pub search_string: String,
    pub hide_hidden_files: bool,

    pub content: FileContent,
    pub history: Vec<PathBuf>,

    pub current_path: PathBuf,
    pub last_path: PathBuf,

    pub app_mode: Modes,

    pub keybinds: KeyBinds,
}

impl Default for FilesApp {
    fn default() -> Self {
        let current_folder = "/home/eko";
        let files = get_root_dir_files(current_folder.into(), true, "".to_string());

        let file_content = FileContent{
                content: "".to_string(),
                file_type: crate::file_man::FileContentType::Dir,
                read: true
        };
        let keybinds = KeyBinds {
            debug: egui::Key::Comma,
            move_back: egui::Key::Minus,
            reset_search: egui::Key::Q,
            search: egui::Key::F,
            hide_hidden_files: egui::Key::B,
            preview: egui::Key::P,
            create: egui::Key::A,
            delete: egui::Key::D,
            rename: egui::Key::R,
            move_in: egui::Key::L,
            move_out: egui::Key::H,
            move_up: egui::Key::K,
            move_down: egui::Key::J,
        };
        Self {
            selected_element_index: 0,
            selected_element: files[0].clone(),

            files,
            preview: true,

            new_file_name: "".to_string(),
            search_string: "".to_string(),
            hide_hidden_files: true,

            content: file_content,
            history: vec![current_folder.into()],

            current_path: PathBuf::from(current_folder),
            last_path: PathBuf::from(current_folder),

            app_mode: Modes::Action,
            keybinds,
        }
    }
}
