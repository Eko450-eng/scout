use std::{ffi::OsString, fs, path::PathBuf};

use crate::{poups::search_file_popup::reset_search, types::FilesApp};

use super::{
    file_man::{get_content, get_root_dir_files},
    utils::reset_cursor,
};

fn is_folder_empty(folder_path: &PathBuf) -> Result<bool, std::io::Error> {
    let dir_entries = fs::read_dir(folder_path)?;
    for _ in dir_entries {
        return Ok(false); // If any entry is found, the folder is not empty
    }
    Ok(true) // If no entries are found, the folder is empty
}

pub fn move_in(app: &mut FilesApp) {
    if !app.empty {
        let item = app.files[app.selected_element_index].clone();

        let mut image_ext_osstring: Vec<OsString> = vec![];
        for i in app.image_formats.clone() {
            image_ext_osstring.push(OsString::from(i))
        }

        if item.path.is_dir() {
            match is_folder_empty(&item.path) {
                Ok(empty) => {
                    if empty {
                        app.empty = true;
                        let mut currentpath = app.selected_element.path.clone();
                        currentpath.pop();
                        app.last_path = currentpath;
                        app.history.push(item.clone().path);
                        app.files = get_root_dir_files(
                            item.clone().path,
                            app.hide_hidden_files,
                            app.search_string.clone(),
                        );

                        app.current_path = item.clone().path;

                        reset_search(app);

                        app.selected_element_index = 0;
                    } else {
                        let mut currentpath = app.selected_element.path.clone();
                        currentpath.pop();
                        app.last_path = currentpath;
                        app.history.push(item.clone().path);
                        app.files = get_root_dir_files(
                            item.clone().path,
                            app.hide_hidden_files,
                            app.search_string.clone(),
                        );

                        app.current_path = item.clone().path;

                        reset_search(app);

                        app.selected_element_index = 0;
                        app.selected_element = app.files[app.selected_element_index].clone();
                    }
                }
                _ => {}
            }
        } else {
            app.selected_element = app.files[app.selected_element_index].clone();
            if item.path.extension().is_some() {
                let ext = item.path.extension().unwrap();

                if image_ext_osstring.contains(&ext.to_os_string()) {
                    app.content = get_content(app.files[app.selected_element_index].clone().path);
                }
            }
        }
        app.search_string = "".to_string();
    }
}

pub fn move_out(app: &mut FilesApp) {
    app.last_path = app.current_path.clone();
    let mut new_path = app.current_path.clone();
    new_path.pop();
    app.current_path = new_path;

    app.files = get_root_dir_files(
        app.current_path.clone(),
        app.hide_hidden_files,
        app.search_string.clone(),
    );
    if app.files.len() > 0 {
        app.empty = false;
    }
    reset_cursor(app)
}

pub fn move_back(app: &mut FilesApp) {
    let new_path = app.last_path.clone();

    app.files = get_root_dir_files(
        app.last_path.clone(),
        app.hide_hidden_files,
        app.search_string.clone(),
    );
    app.last_path = app.current_path.clone();
    app.current_path = new_path;
    reset_cursor(app)
}

// TODO: Implementation needed
pub fn move_forward(app: &mut FilesApp) {
    let h = &app.history[app.history.len() - 1].clone();
    app.history.push(h.to_path_buf());
    app.files = get_root_dir_files(
        PathBuf::from(h),
        app.hide_hidden_files,
        app.search_string.clone(),
    );
    app.selected_element_index = 0;
}

pub fn move_up(app: &mut FilesApp) {
    if !app.empty && app.selected_element_index > 0 {
        app.selected_element_index = app.selected_element_index - 1;
        app.selected_element = app.files[app.selected_element_index].clone();
        if app.preview {
            app.content = get_content(app.files[app.selected_element_index].clone().path);
        }
    }
}

pub fn move_down(app: &mut FilesApp) {
    if !app.empty && app.selected_element_index < app.files.len() - 1 {
        app.selected_element_index = app.selected_element_index + 1;
        app.selected_element = app.files[app.selected_element_index].clone();
        if app.preview {
            app.content = get_content(app.files[app.selected_element_index].clone().path);
        }
    }
}
