use std::{fs::read_dir, path::{Path, PathBuf}};

use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};

slint::include_modules!();

pub fn get_folders_list(path: PathBuf) -> ModelRc<StandardListViewItem> {
    return slint::ModelRc::new(VecModel::from(get_root_dir_files(path)));
}

pub fn get_current_folder() -> PathBuf {
    if cfg!(target_os = "windows") {
        Path::new("C:\\").to_path_buf()
    } else {
        Path::new("/home").to_path_buf()
    }
}

pub fn get_root_dir_files(dir: PathBuf) -> Vec<StandardListViewItem> {
    let file = read_dir(dir)
        .expect("Fail")
        .enumerate()
        .filter_map(|(_index, entry)| {
            entry.ok().and_then(|e| {
                e.path().file_name().and_then(|n| {
                    n.to_str().map(|s| StandardListViewItem::from(SharedString::from(s)))
                })
            })
        })
        .collect::<Vec<StandardListViewItem>>();

    return file;
}
