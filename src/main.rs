use std::path::{Path, PathBuf};
use std::{env, fs};

use slint::{SharedString, VecModel};

const _APP_ID: &str = "de.wipdesign.scout";
const _APP_NAME: &str = "Scout";
const _APP_VERSION: &str = "0.0.1";

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let files = get_root_dir_files(get_current_folder());
    let files_list = slint::ModelRc::new(VecModel::from(files));
    ui.set_files(files_list);

    ui.on_set_active_folder({
        let ui_handle = ui.as_weak();
        move |data| {
            let ui = ui_handle.unwrap();
            let mut new_path = PathBuf::new();

            let path_append: String = data.clone().into();

            new_path.push("/home/eko/.config/nvim/");
            new_path.push(path_append);

            println!("DATA: {:?} \nPATH: {:?}", data, new_path);

            if new_path.is_dir() {
                let child_files = get_root_dir_files(new_path);

                let child_files_list = slint::ModelRc::new(VecModel::from(child_files));

                ui.set_child_files(child_files_list);
            }else if new_path.is_file(){
                let c = fs::read_to_string(new_path).expect("Failed to Read file");

                let content = SharedString::from(c);

                ui.set_content_of_file(content)
            }else {
                return
            }
        }
    });

    ui.run()
}

fn get_current_folder() -> PathBuf {
    if cfg!(target_os = "windows") {
        Path::new("C:\\").to_path_buf()
    } else {
        Path::new("/home/eko/.config/nvim").to_path_buf()
    }
}

fn get_root_dir_files(dir: PathBuf) -> Vec<SharedString> {
    let file = fs::read_dir(dir)
        .expect("Fail")
        .filter_map(|paths| {
            paths.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| SharedString::from(s)))
            })
        })
        .collect::<Vec<SharedString>>();

    return file;
}
