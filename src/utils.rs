use crate::FilesApp;

pub fn reset_cursor(app: &mut FilesApp) {
    app.selected_element_index = 0;
    app.selected_element = app.files[0].clone();
}
