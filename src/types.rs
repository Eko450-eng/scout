use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use struct_iterable::Iterable;

use crate::scout_utils::file_man::FileContent;
use crate::scout_utils::file_man::{get_root_dir_files, FileContentType};
use crate::scout_utils::utils::get_home_dir;

#[derive(Debug, Iterable, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub enum Modes {
    Action,
    Editing,
    Creation,
    Search,
    Renaming,
    Deletion,
    NonAction,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ItemElement {
    pub name: String,
    pub path: PathBuf,
    pub _dir: bool,
}

// FilesApp
#[derive(Debug, Deserialize, Serialize)]
pub struct FilesApp {
    pub seperator: String,
    pub selected_element_index: usize,
    pub selected_element: ItemElement,

    pub files: Vec<ItemElement>,
    pub preview: bool,

    pub target: String,

    pub new_file_name: String,
    pub search_string: String,
    pub hide_hidden_files: bool,

    pub image_formats: Vec<String>,
    pub content: FileContent,
    pub history: Vec<PathBuf>,

    pub empty: bool,
    pub current_path: PathBuf,
    pub last_path: PathBuf,

    pub app_mode: Modes,

    pub keybinds: KeyBinds,

    pub debug: bool,
    pub setting: bool,

    pub double_g: bool,
    pub counter: i32,
}

impl Default for FilesApp {
    fn default() -> Self {
        let current_folder = get_home_dir();
        let files = get_root_dir_files(current_folder.clone(), true, "".to_string());

        let file_content = FileContent {
            content: "".to_string(),
            file_type: FileContentType::Dir,
            read: true,
        };

        let seperator = if std::env::consts::OS == "windows" {
            "\\".to_string()
        } else {
            "/".to_string()
        };

        let image_formats: Vec<String> = vec![
            "png".to_string(),
            "jpg".to_string(),
            "jpeg".to_string(),
            "JPEG".to_string(),
            "JPG".to_string(),
        ];

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
            seperator,
            selected_element_index: 0,
            selected_element: files[0].clone(),

            files,
            preview: true,

            target: "".to_string(),

            new_file_name: "".to_string(),
            search_string: "".to_string(),
            hide_hidden_files: true,

            image_formats,
            content: file_content,
            history: vec![current_folder.clone()],

            empty: false,
            current_path: PathBuf::from(current_folder.clone()),
            last_path: PathBuf::from(current_folder),

            app_mode: Modes::Action,
            keybinds,

            debug: false,
            setting: false,

            double_g: false,
            counter: 0,
        }
    }
}
