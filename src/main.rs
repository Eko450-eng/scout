#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod file_man;
mod types;
use std::{path::PathBuf, usize};

use eframe::{egui, App};
use egui::{popup_below_widget, Id, PopupCloseBehavior, ScrollArea};
use egui_code_editor::{CodeEditor, Syntax};
use file_man::{add_file, get_content, get_root_dir_files, rename_file};
use types::{ItemElement, KeyBinds, Modes};

pub struct FilesApp {
    selected_element_index: usize,
    selected_element: ItemElement,

    files: Vec<ItemElement>,
    preview: bool,

    content: String,
    history: Vec<PathBuf>,

    app_mode: Modes,
}

impl Default for FilesApp {
    fn default() -> Self {
        let current_folder = "/home/eko";
        let files = get_root_dir_files(current_folder.into());
        Self {
            selected_element_index: 0,
            selected_element: files[0].clone(),

            files,
            preview: false,

            content: String::new(),
            history: vec![current_folder.into()],

            app_mode: Modes::Action,
        }
    }
}

impl App for FilesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut text = self.selected_element.path.to_string_lossy().to_string();
        egui::CentralPanel::default().show(ctx, |ui| {
            let heading;
            match self.app_mode {
                Modes::Action => heading = "Action",
                Modes::Editing => heading = "Editing",
                Modes::Creation => heading = "Creation",
            }
            ui.heading(heading);

            let keybinds = KeyBinds {
                create: egui::Key::A,
                delete: egui::Key::D,
                rename: egui::Key::R,
                move_in: egui::Key::L,
                move_out: egui::Key::H,
                move_up: egui::Key::K,
                move_down: egui::Key::J,
            };

            // Moving out of directory
            let back_button = ui.button("<");
            if back_button.clicked() {
                let h = &self.history[self.history.len() - 2].clone();
                self.history.push(h.to_path_buf());
                self.files = get_root_dir_files(PathBuf::from(h));
            }

            let response = ui.button("Open");

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    if ui.input(|i| i.key_pressed(egui::Key::I)) {
                        match self.app_mode {
                            Modes::Action => self.app_mode = Modes::Editing,
                            Modes::Editing => self.app_mode = Modes::Creation,
                            Modes::Creation => self.app_mode = Modes::Action,
                        }
                    }

                    match self.app_mode {
                        Modes::Action => {
                            if ui.input(|i| i.key_pressed(keybinds.create)) {
                                self.app_mode = Modes::Creation
                            } else if ui.input(|i| i.key_pressed(keybinds.delete)) {
                            } else if ui.input(|i| i.key_pressed(keybinds.rename)) {
                                if false == true {
                                    rename_file(
                                        self.selected_element.path.clone(),
                                        "test".to_string(),
                                    )
                                } else {
                                    println!("This feature is currently disabled")
                                }
                            } else if ui.input(|i| {
                                i.key_pressed(keybinds.move_down)
                                    && self.selected_element_index < self.files.len() - 1
                            }) {
                                self.selected_element_index = self.selected_element_index + 1;
                                self.selected_element =
                                    self.files[self.selected_element_index].clone();
                                if self.preview {
                                    self.content = get_content(
                                        self.files[self.selected_element_index].clone().path,
                                    );
                                }
                            } else if ui.input(|i| i.key_pressed(keybinds.move_up))
                                && self.selected_element_index > 0
                            {
                                self.selected_element_index = self.selected_element_index - 1;
                                self.selected_element =
                                    self.files[self.selected_element_index].clone();
                                if self.preview {
                                    self.content = get_content(
                                        self.files[self.selected_element_index].clone().path,
                                    );
                                }
                                println!(
                                    "I: {:?} \nF: {:?}",
                                    self.selected_element_index,
                                    self.files.len()
                                );
                            } else if ui.input(|i| i.key_pressed(keybinds.move_out)) {
                                let h = &self.history[self.history.len() - 2].clone();
                                self.history.push(h.to_path_buf());
                                self.files = get_root_dir_files(PathBuf::from(h));
                                self.selected_element_index = 0;
                            } else if ui.input(|i| {
                                (i.key_pressed(egui::Key::Enter)) || i.key_pressed(keybinds.move_in)
                            }) {
                                let item = self.files[self.selected_element_index].clone();

                                if item.path.is_dir() {
                                    self.history.push(item.clone().path);
                                    self.files = get_root_dir_files(item.clone().path);
                                } else {
                                    self.selected_element =
                                        self.files[self.selected_element_index].clone();
                                    self.content = get_content(
                                        self.files[self.selected_element_index].clone().path,
                                    );
                                }
                                self.selected_element_index = 0;
                            };
                        }
                        Modes::Creation => {
                            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                let mut path = self.selected_element.clone().path;
                                path.pop();

                                add_file(path, text.clone());
                                println!("CREATED: {:?}", text)
                            } else if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                self.app_mode = Modes::Action
                            }
                        }
                        _ => {
                            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                println!("New Text: {:?}", self.content)
                            }
                        }
                    };

                    let popup_id = Id::new("popup_id");

                    if response.clicked() {
                        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                        self.app_mode = Modes::Creation
                    }
                    popup_below_widget(
                        ui,
                        popup_id,
                        &response,
                        PopupCloseBehavior::CloseOnClickOutside,
                        |ui| {
                            ui.set_min_width(300.0);
                            ui.label("This popup will be open even if you click the checkbox");
                            ui.text_edit_singleline(&mut text);
                        },
                    );

                    let vh = ctx.input(|i| i.screen_rect().y_range());
                    ui.set_height(vh.max - 100.0);

                    // Current folder view
                    ScrollArea::vertical()
                        // .max_height(vh_with_padding - 50.0)
                        .show(ui, |ui| {
                            for (index, item) in self.files.clone().iter().enumerate() {
                                let i = ui.selectable_label(
                                    index == self.selected_element_index,
                                    &item.name,
                                );
                                if i.double_clicked() {
                                    self.history.push(item.clone().path);
                                    println!("Got into {:?}", item.path);
                                    self.files = get_root_dir_files(item.clone().path);
                                } else if i.clicked() {
                                    self.selected_element = item.clone();
                                    self.selected_element_index = index;
                                    self.content = get_content(item.clone().path)
                                }
                            }
                        });
                });

                ui.vertical(|ui| {
                    // Selected Content view
                    if self.selected_element.dir {
                        CodeEditor::default()
                            .id_source("code_editor")
                            .with_rows(12)
                            .with_fontsize(12.0)
                            .with_syntax(Syntax::default())
                            .with_numlines(false)
                            .show(ui, &mut self.content)
                    } else {
                        CodeEditor::default()
                            .id_source("code_editor")
                            .with_rows(12)
                            .with_fontsize(12.0)
                            .with_syntax(Syntax::rust())
                            .with_numlines(true)
                            .show(ui, &mut self.content)
                    }
                });
            });

            // ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
        });
    }
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
                        //
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([300.0, 200.0]),
        ..Default::default()
    };

    // Our application state:

    //eframe::run_simple_native("My egui App", options, move |ctx, _frame| { });
    eframe::run_native(
        "Scout",
        options,
        Box::new(|_cc| Ok(Box::new(FilesApp::default()))),
    )
}
