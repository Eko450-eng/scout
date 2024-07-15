use std::{ffi::OsString, path::PathBuf};

use crate::{file_man::{get_content, get_root_dir_files}, search_file_popup::reset_search, utils::reset_cursor, FilesApp};

pub fn move_in(app: &mut FilesApp) {
    let item = app.files[app.selected_element_index].clone();

    let image_ext = "png";
    let os_string: OsString = OsString::from(image_ext);

    if item.path.is_dir() {
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
        app.selected_element_index = 0;

        reset_search(app);
    } else {
        app.selected_element = app.files[app.selected_element_index].clone();
        if item.path.extension() != Some(&os_string) {
            app.content = get_content(app.files[app.selected_element_index].clone().path);
        }
    }
    app.search_string = "".to_string();
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
    app.selected_element_index = app.selected_element_index - 1;
    app.selected_element = app.files[app.selected_element_index].clone();
    if app.preview {
        app.content = get_content(app.files[app.selected_element_index].clone().path);
    }
}

pub fn move_down(app: &mut FilesApp) {
    app.selected_element_index = app.selected_element_index + 1;
    app.selected_element = app.files[app.selected_element_index].clone();
    if app.preview {
        app.content = get_content(app.files[app.selected_element_index].clone().path);
    }
}
