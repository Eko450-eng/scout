use std::path::PathBuf;
use slint::Weak;

use crate::slint_generatedAppWindow::AppWindow;

pub fn get_last_path(ui: Weak<AppWindow>) -> PathBuf {
    return PathBuf::from(ui.unwrap().get_last_path().to_string());
}

