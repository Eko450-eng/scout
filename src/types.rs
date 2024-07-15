use std::path::PathBuf;

pub struct KeyBinds {
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
}

#[derive(Clone, Default)]
pub struct ItemElement {
    pub name: String,
    pub path: PathBuf,
    pub dir: bool,
}
