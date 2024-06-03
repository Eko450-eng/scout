use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

use slint::{ModelRc, SharedString, VecModel};

const _APP_ID: &str = "de.wipdesign.scout";
const _APP_NAME: &str = "Scout";
const _APP_VERSION: &str = "0.0.1";

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

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
    // TODO: Manage how to move with keys and also move in AND out without reloading the program...
    ui.on_set_active_folder({
        let ui_handle = ui.as_weak();

        move |data| {
            let ui = ui_handle.unwrap();

            let mut new_path = history[depth].clone();

            let path_append: String = data.clone().into();
            new_path.push(path_append);

            let ext = new_path.extension();

            if new_path.is_dir() {
                history.push(new_path);
                let values = set_view(history.clone(), depth, depth + 1);

                depth += 1;
                ui.set_files(values.0);
                ui.set_child_files(values.1);
            } else if ext.unwrap() == "lua" {
                // let c = fs::read_to_string(new_path.clone()).expect("Failed to Read file");

                // let content = SharedString::from(c);
                // TODO: Properly check for multiple file types

                Command::new("wezterm")
                    .arg("start")
                    .arg("nvim")
                    .arg(&new_path)
                    .status()
                    .expect("Failed to open");

                // ui.set_content_of_file(content)
            } else {
                return;
            }
        }
    });

    // Startup
    ui.run()
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
