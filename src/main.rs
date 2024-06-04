use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs, usize};

use slint::{Model, ModelRc, SharedString, VecModel};

const _APP_ID: &str = "de.wipdesign.scout";
const _APP_NAME: &str = "Scout";
const _APP_VERSION: &str = "0.0.1";

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
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

    ui.set_files(parent_files_list);
    ui.set_child_files(files_list);

    let mut depth = 1;

    // Handle Interaction
    let up = SharedString::from("k");
    let down = SharedString::from("j");
    let into = SharedString::from("l");
    let outof = SharedString::from("h");
    let mut data_vec: Vec<SolItem> = Vec::new();

    window.on_key_presed(move |key, data| {
        let current_position = ui.get_position();
        let mut new_y = current_position.y;
        for i in data.iter() {
            data_vec.push(i);
        }

        if key == up {
            new_y = current_position.y - 1;
        } else if key == down {
            new_y = current_position.y + 1;

        // ╭─────────────────╮
        // │ Move Out Folder │
        // ╰─────────────────╯
        } else if key == outof {
            let mut new_path = history[depth].clone();

            new_path.pop();

            if new_path.is_dir() {
                history.push(new_path);
                let values = set_view(history.clone(), depth, depth - 1);

                depth -= 1;
                ui.set_files(values.0);
                ui.set_child_files(values.1);
            } else {
                return;
            }

        // ╭──────────────────╮
        // │ Move Into Folder │
        // ╰──────────────────╯
        } else if key == into {
            let mut new_path = history[depth].clone();

            let path_append: String = data_vec[new_y as usize].name.clone().into();
            new_path.push(path_append);

            if new_path.is_dir() {
                history.push(new_path);
                let values = set_view(history.clone(), depth, depth + 1);

                depth += 1;
                ui.set_files(values.0);
                ui.set_child_files(values.1);
            } else {
                let ext = new_path.extension().unwrap();
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
                return;
            }
        }

        let new_pos: slint_generatedAppWindow::pos = pos {
            y: new_y,
            x: current_position.x,
        };

        ui.set_position(new_pos);
    });

    // Startup
    window.run()
}

fn set_view(history: Vec<PathBuf>, depth: usize, c: usize) -> (ModelRc<SolItem>, ModelRc<SolItem>) {
    let parent_files_list =
        slint::ModelRc::new(VecModel::from(get_root_dir_files(history[depth].clone())));
    let files_list = slint::ModelRc::new(VecModel::from(get_root_dir_files(history[c].clone())));

    return (parent_files_list, files_list);
}

fn get_current_folder() -> PathBuf {
    if cfg!(target_os = "windows") {
        Path::new("C:\\").to_path_buf()
    } else {
        Path::new("/home/eko/.config/nvim").to_path_buf()
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
