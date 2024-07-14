#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::{
    fs::{self, read_dir, File},
    io::{self, BufReader, Read, Write},
    path::{Path, PathBuf},
};

use eframe::egui;
use egui::{popup_below_widget, Id, PopupCloseBehavior, ScrollArea};
use egui_code_editor::{CodeEditor, Syntax};

pub struct KeyBinds {
    create: egui::Key,
    delete: egui::Key,
    rename: egui::Key,
    move_in: egui::Key,
    move_out: egui::Key,
    move_up: egui::Key,
    move_down: egui::Key,
}

pub enum Modes {
    Action,
    Editing,
    Creation,
}

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
    file.retain(|item| !item.name.starts_with("."));

    return file;
}

fn is_utf8<P: AsRef<Path>>(path: P) -> io::Result<bool> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1024];

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        if let Err(_) = std::str::from_utf8(&buffer[..bytes_read]) {
            return Ok(false);
        }
    }

    Ok(true)
}

fn get_content(target: PathBuf) -> String {
    let mut files_list: Vec<String> = vec![];
    if target.is_file() {
        match is_utf8(target.clone()) {
            Ok(true) => fs::read_to_string(target.clone()).expect("Failed to Read file"),
            Ok(false) => "Hi".to_string(),
            Err(_) => "Error".to_string(),
        }
    } else {
        let items = get_root_dir_files(target);
        for i in items {
            files_list.push(i.name)
        }
        return files_list.join("\n");
    }
}

pub fn rename_file(file: PathBuf, target_string: String) {
    let original_file_name = file.clone();

    let mut original_file_parent_location = file.clone();
    original_file_parent_location.pop();

    let mut new_file_name = original_file_parent_location.clone();
    new_file_name.push(target_string);

    println!("Original name: {:?}", original_file_name);
    println!("Original location: {:?}", original_file_parent_location);
    println!("New name: {:?}", new_file_name);

    match fs::rename(original_file_name.clone(), new_file_name.clone()) {
        Ok(_) => {
            println!("Moved from {:?} to {:?}", original_file_name, new_file_name)
        }
        Err(err) => {
            println!(
                "Could not Rename / Move from {:?} to {:?} because: {:?}",
                original_file_name, new_file_name, err
            )
        }
    }
}


pub fn add_file(target: PathBuf, name: String) {
    let content: &[u8] = b"";
    if target.is_dir() {
        let mut target_item = target.clone();
        target_item.push(name.to_string());

        // Check wether it is a path or file name
        if target_item.to_str().unwrap().contains("/") {
            let mut final_file_name: PathBuf = PathBuf::new();
            let mut create_file = false;

            // Check if last element is a file or still a path
            if !name.ends_with("/") {
                final_file_name = name.clone().split("/").last().unwrap().into();
                target_item.pop();
                create_file = true;
            }

            // Create all paths
            let dir = fs::create_dir_all(target_item.clone());

            if dir.is_ok() {
                // If last item was a file create the file inside the path
                if create_file {
                    target_item.push(final_file_name);
                    let file = File::create(target_item.clone());
                    if file.is_ok() {
                        file.unwrap().write_all(content).unwrap();
                    } else {
                        println!(
                            "Failed to Create file at {:?}\nBecause: {:?}",
                            target_item, file
                        );
                    }
                }
                println!("Created directory: {:?}", dir)
            } else {
                println!(
                    "Failed to Create directory {:?}\nBecause: {:?}",
                    target_item, dir
                );
            }
        } else {
            let file = File::create(target_item.clone());
            if file.is_ok() {
                file.unwrap().write_all(content).unwrap();
            } else {
                println!(
                    "Failed to Create file at {:?}\nBecause: {:?}",
                    target_item, file
                );
            }
        }

        // if !file.is_ok() {
        //     println!(
        //         "Failed to Create file at {:?}\nBecause: {:?}",
        //         file_name, file
        //     );
        // }
        println!("Create file at {:?}", target_item);
    } else {
        println!("Target is not a dir")
    }
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
                        //
    let current_folder = "/home/eko/Desktop";
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
    selected_element = files[0].clone();

    let mut content = String::new();
    let mut history: Vec<PathBuf> = vec!["/home/eko/Desktop".into()];

    let mut app_mode = Modes::Action;
    let mut text = selected_element.path.to_string_lossy().to_string();

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let heading;
            match app_mode {
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
                let h = &history[history.len() - 2].clone();
                history.push(h.to_path_buf());
                files = get_root_dir_files(PathBuf::from(h));
            }

            let response = ui.button("Open");

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    if ui.input(|i| i.key_pressed(egui::Key::I)) {
                        match app_mode {
                            Modes::Action => app_mode = Modes::Editing,
                            Modes::Editing => app_mode = Modes::Creation,
                            Modes::Creation => app_mode = Modes::Editing,
                        }
                    }

                    match app_mode {
                        Modes::Action => {
                            if ui.input(|i| i.key_pressed(keybinds.create)) {
                                app_mode = Modes::Creation
                            } else if ui.input(|i| i.key_pressed(keybinds.delete)) {
                            } else if ui.input(|i| i.key_pressed(keybinds.rename)) {
                                if false == true {
                                    rename_file(selected_element.path.clone(), "test".to_string())
                                } else {
                                    println!("This feature is currently disabled")
                                }
                            } else if ui.input(|i| i.key_pressed(keybinds.move_down)) {
                                selected_element_index = selected_element_index + 1;
                                selected_element = files[selected_element_index].clone();
                                content = get_content(files[selected_element_index].clone().path);
                            } else if ui.input(|i| i.key_pressed(keybinds.move_up)) {
                                selected_element_index = selected_element_index - 1;
                                selected_element = files[selected_element_index].clone();
                                content = get_content(files[selected_element_index].clone().path);
                            } else if ui.input(|i| i.key_pressed(keybinds.move_out)) {
                                let h = &history[history.len() - 2].clone();
                                history.push(h.to_path_buf());
                                files = get_root_dir_files(PathBuf::from(h));
                                selected_element_index = 0;
                            } else if ui.input(|i| {
                                (i.key_pressed(egui::Key::Enter)) || i.key_pressed(keybinds.move_in)
                            }) {
                                let item = files[selected_element_index].clone();

                                if item.path.is_dir() {
                                    history.push(item.clone().path);
                                    files = get_root_dir_files(item.clone().path);
                                } else {
                                    selected_element = files[selected_element_index].clone();
                                    content =
                                        get_content(files[selected_element_index].clone().path);
                                }
                                selected_element_index = 0;
                            };
                        }
                        Modes::Creation => {
                            if ui.input(|i| i.key_pressed(egui::Key::Enter)){
                                let mut path = selected_element.clone().path;
                                path.pop();

                                add_file(path, text.clone());
                                println!("CREATED: {:?}", text)
                            }else if ui.input(|i| i.key_pressed(egui::Key::Escape)){
                                app_mode = Modes::Action
                            }
                        }
                        _ => {
                            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                println!("New Text: {:?}", content)
                            }
                        }
                    };

                    let popup_id = Id::new("popup_id");
                    
                    if response.clicked() {
                        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                        app_mode =  Modes::Creation
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
                    let vh_with_padding = vh.max - 20.0;
                    ui.set_height(vh_with_padding);

                    // Current folder view
                    ScrollArea::vertical()
                        .max_height(vh_with_padding - 20.0)
                        .show(ui, |ui| {
                            for (index, item) in files.clone().iter().enumerate() {
                                let i = ui
                                    .selectable_label(index == selected_element_index, &item.name);
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
