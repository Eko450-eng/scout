use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs, usize};

use slint::{Model, ModelRc, SharedString, StandardListViewItem, VecModel};

const _APP_ID: &str = "de.wipdesign.scout";
const _APP_NAME: &str = "Scout";
const _APP_VERSION: &str = "0.0.1";

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // Constants
    let window = AppWindow::new()?;
    let ui_handle = window.as_weak();
    let ui = ui_handle.unwrap();

    // Set defaults
    let mut history: Vec<PathBuf> = [].to_vec();
    let mut root = get_current_folder();
    let current_path = root.clone();

    root.pop();
    history.push(root);
    history.push(current_path);

    let parent_files_list =
        slint::ModelRc::new(VecModel::from(get_root_dir_files(history[0].clone())));
    let files_list = slint::ModelRc::new(VecModel::from(get_root_dir_files(history[1].clone())));

    let mut ii = vec![];

    for i in files_list.iter() {
        let it = StandardListViewItem::from(i.name);
        ii.push(it);
    }

    let current_view = ModelRc::new(VecModel::from(ii.clone()));
    ui.set_files(parent_files_list);

    set_child(ui.clone_strong(), files_list.clone());
    // let pos = window.get_child_pos();

    let mut depth = 1;

    // Handle Interaction
    let up = SharedString::from("k");
    let down = SharedString::from("j");
    let into = SharedString::from("l");
    let outof = SharedString::from("h");
    let find = SharedString::from("f");
    let quit = SharedString::from("q");

    window.on_key_presed(move |key_event| {
        let key = key_event.text;
        println!(
            "
        {:?}
        \n{:?}
        \n{:?}
        \n{:?}
                 ",
            ui.get_files().row_data(ui.get_child_pos() as usize),
            ui.get_child_pos(),
            ui.get_child_files_standard()
                .row_data(ui.get_child_pos() as usize),
            ui.get_position()
        );

        // TODO: Pagination instead of / or additional to scrolling
        if key == up {
            move_y(ui_handle.unwrap(), "up".to_string())
        } else if key == down {
            move_y(ui_handle.unwrap(), "down".to_string())
        } else if key == quit {
            set_child(ui.clone_strong(), files_list.clone());
            ui.set_position(pos { x: 0, y: 0 })
        } else if key == find {
            let mut ii = vec![];

            for name in current_view.iter() {
                let f = ui.get_find_value().to_string();
                if name.text.contains(&f) {
                    let it = StandardListViewItem::from(name.text);
                    ii.push(it);
                }
            }

            let cfs = ModelRc::new(VecModel::from(ii.clone()));

            ui.set_child_files_standard(cfs);
        } else if key == outof {
            let (y, x, d) = move_out(ui_handle.unwrap(), &mut history, depth);
            let new_pos = pos { x, y };
            ui.set_position(new_pos);
            depth = d;
        } else if key == into {
            let (y, x, d) = move_in(ui_handle.unwrap(), &mut history, depth);
            let new_pos = pos { x, y };
            ui.set_position(new_pos);
            depth = d;
        }
    });

    // Startup
    window.run()
}

fn get_folders_list(path: PathBuf) -> ModelRc<SolItem> {
    return slint::ModelRc::new(VecModel::from(get_root_dir_files(path)));
}

fn get_current_folder() -> PathBuf {
    if cfg!(target_os = "windows") {
        Path::new("C:\\").to_path_buf()
    } else {
        Path::new("/home").to_path_buf()
    }
}

fn get_root_dir_files(dir: PathBuf) -> Vec<SolItem> {
    let file = fs::read_dir(dir)
        .expect("Fail")
        .enumerate()
        .filter_map(|(index, entry)| {
            entry.ok().and_then(|e| {
                e.path().file_name().and_then(|n| {
                    n.to_str().map(|s| SolItem {
                        // TODO: Add Icons and Colors and such
                        index: index.try_into().unwrap(),
                        name: SharedString::from(s),
                        item_type: SolItemType::File,
                        active: false,
                        selected: false,
                        path: SharedString::from(s),
                    })
                })
            })
        })
        .collect::<Vec<SolItem>>();

    return file;
}

fn set_child(ui: AppWindow, files_list: ModelRc<SolItem>) {
    let mut ii = vec![];

    for i in files_list.iter() {
        let it = StandardListViewItem::from(i.name);
        ii.push(it);
    }
    let cfs = ModelRc::new(VecModel::from(ii.clone()));

    ui.set_child_files_standard(cfs.clone());
}

fn move_y(ui: AppWindow, dir: String) {
    println!("MOVING");
    let mut current_position = ui.get_position().clone();

    if dir == "up" {
        current_position.y -= 1;
    } else if dir == "down" {
        current_position.y += 1;
    }
    ui.set_position(pos {
        x: current_position.x,
        y: current_position.y,
    })
}

fn move_in(ui: AppWindow, history: &mut Vec<PathBuf>, depth: i32) -> (i32, i32, i32) {
    let mut new_path = history.last().expect("No last Element").clone();
    let data = ui
        .get_child_files_standard()
        .row_data(ui.get_child_pos() as usize);
    let name = Some(data).unwrap_or_default();
    new_path.push(name.unwrap().text.to_string());

    if new_path.is_dir() {
        history.push(new_path);

        let mut child = history[0].clone();

        if history[history.len() - 1].exists() {
            child = history[history.len() - 1].clone();
        }

        let mut parent = history[0].clone();
        if history[history.len() - 2].exists() {
            parent = history[history.len() - 2].clone();
        }
        let parent_files_list = get_folders_list(parent);
        ui.set_files(parent_files_list);

        let files_list = get_folders_list(child);

        set_child(ui, files_list);

        return (
            // TODO: Handle X coordinates
            0,
            0,
            depth + 1,
        );
    } else {
        // TODO: Handle no extension files
        let new_path_ext = new_path
            .extension()
            .expect(&format!("Failed to read extension of: {:?}", new_path));
        let ext = new_path_ext;
        let ext_str = ext.to_str();
        match ext_str {
            Some("lua") => {
                Command::new("wezterm")
                    .arg("start")
                    .arg("nvim")
                    .arg(&new_path)
                    .status()
                    .expect("Failed to open");
            }
            _ => {
                let c = fs::read_to_string(new_path.clone()).expect("Failed to Read file");

                let content = SharedString::from(c);
                // TODO: Properly check for multiple file types

                ui.set_content_of_file(content)
            }
        }
        return (
            // TODO: Handle X coordinates
            ui.get_position().y,
            ui.get_position().x,
            depth,
        );
    }
}

fn move_out(ui: AppWindow, history: &mut Vec<PathBuf>, depth: i32) -> (i32, i32, i32) {
    let mut new_path = history[depth as usize].clone();

    new_path.pop();

    if new_path.is_dir() {
        if history.len() > 1 {
            history.pop();
        }

        let mut child = history[0].clone();

        // let mut parent = history[0].clone();
        // if history[history.len() - 2].exists() {
        //     parent = history[history.len() - 2].clone();
        // }
        // let parent_files_list = get_folders_list(parent);

        if history[history.len() - 1].exists() {
            child = history[history.len() - 1].clone();
        }

        let files_list = get_folders_list(child);

        set_child(ui, files_list);

        // ui.set_files(parent_files_list);

        return (0, 0, depth - 1);
    } else {
        return (ui.get_position().y, ui.get_position().x, depth);
    }
}
