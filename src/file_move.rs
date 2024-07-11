use std::{
    fs::{self, read_dir, File},
    io::Write,
    path::{Path, PathBuf},
};

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
                    n.to_str()
                        .map(|s| StandardListViewItem::from(SharedString::from(s)))
                })
            })
        })
        .collect::<Vec<StandardListViewItem>>();

    return file;
}

pub fn create_file(target: PathBuf, name: SharedString) {
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

pub fn rename_file(file: PathBuf, target_string: SharedString) {
    let mut target = PathBuf::new();
    target.push(target_string.to_string());

    match fs::rename(file.clone(), target.clone()) {
        Ok(_) => {
            println!("Moved from {:?} to {:?}", file, target)
        }
        Err(err) => {
            println!(
                "Could not Rename / Move from {:?} to {:?} because: {:?}",
                file, target, err
            )
        }
    }
}

pub fn delete_file(target: PathBuf) -> bool{
    if target.is_dir() {
        match fs::remove_dir_all(target.clone()) {
            Ok(_) => return false,
            Err(err) => println!("Could not delete: {:?} \nBecause {:?}", target, err),
        }
    } else if target.is_file() {
        match fs::remove_file(target.clone()) {
            Ok(_) => return false,
            Err(err) => println!("Could not delete: {:?} \nBecause {:?}", target, err),
        }
    } else {
        println!("Could not delete: {:?}", target);
    }
    return false;
}
