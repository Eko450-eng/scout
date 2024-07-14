#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::{
    fs::{self, read_dir},
    path::PathBuf,
};

use eframe::egui;
use egui::ScrollArea;
use egui_code_editor::{CodeEditor, Syntax};

#[derive(Clone)]
pub struct ItemElement {
    name: String,
    path: PathBuf,
    dir: bool,
}

pub fn get_root_dir_files(dir: PathBuf) -> Vec<ItemElement> {
    let mut file: Vec<ItemElement> = read_dir(dir)
        .expect("Fail")
        .enumerate()
        .filter_map(|(_index, entry)| {
            entry.ok().and_then(|en| {
                Some(ItemElement {
                    name: en.file_name().into_string().expect("No file name found"),
                    path: en.path(),
                    dir: en.path().is_dir(),
                })
            })
        })
        .collect();

        file.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    return file;
}

fn get_content(target: PathBuf) -> String {
    let mut files_list: Vec<String> = vec![];
    if target.is_file() {
        fs::read_to_string(target.clone()).expect("Failed to Read file")
    } else {
        let items = get_root_dir_files(target);
        for i in items {
            files_list.push(i.name)
        }
        return files_list.join("\n");
    }
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
                        //
    let current_folder = "/home/eko";
    let mut files = get_root_dir_files(current_folder.into());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // Our application state:
    let mut selected_element_index = 2;
    let mut selected_element: ItemElement = ItemElement {
        name: "".into(),
        path: PathBuf::from(""),
        dir: false,
    };
    let mut content = String::new();
    let mut history: Vec<PathBuf> = vec!["/home/eko".into()];

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(current_folder);

            // Moving out of directory
            let back_button = ui.button("<");
            if back_button.clicked() {
                let h = &history[history.len() - 2].clone();
                history.push(h.to_path_buf());
                files = get_root_dir_files(PathBuf::from(h));
            }

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    let vh = ctx.input(|i| i.screen_rect().y_range());
                    let vh_with_padding = vh.max - 20.0;
                    ui.set_height(vh_with_padding);

                    // Current folder view
                    ScrollArea::vertical().max_height(vh_with_padding - 20.0).show(ui, |ui| {
                        for (index, item) in files.clone().iter().enumerate() {
                            let i =
                                ui.selectable_label(index == selected_element_index, &item.name);
                            if i.double_clicked() {
                                history.push(item.clone().path);
                                println!("Got into {:?}", item.path);
                                files = get_root_dir_files(item.clone().path);
                            } else if i.clicked() {
                                selected_element = item.clone();
                                selected_element_index = index;
                                content = get_content(item.clone().path)
                            }
                        }
                    });
                });

                ui.vertical(|ui| {
                    // Selected Content view
                    if selected_element.dir {
                        CodeEditor::default()
                            .id_source("code_editor")
                            .with_rows(12)
                            .with_fontsize(12.0)
                            .with_syntax(Syntax::default())
                            .with_numlines(false)
                            .show(ui, &mut content)
                    } else {
                        CodeEditor::default()
                            .id_source("code_editor")
                            .with_rows(12)
                            .with_fontsize(12.0)
                            .with_syntax(Syntax::rust())
                            .with_numlines(true)
                            .show(ui, &mut content)
                    }
                });
            });

            // ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
        });
    })
}
