use std::{env, fs, path::PathBuf};

use serde_json::json;

use crate::{types::ItemElement, FilesApp};

/// Sets the selected_element_index to 0 and changes the selected element accordingly
pub fn reset_cursor(app: &mut FilesApp) {
    app.selected_element_index = 0;
    app.selected_element = app.files[0].clone();
}

const CONFIG_NAME: &str = "config.json";

pub fn get_home_dir() -> PathBuf {
    let config_location = if std::env::consts::OS == "windows" {
        "C:\\\\".to_string()
    } else {
        env::var("HOME").unwrap()
    };
    return PathBuf::from(config_location);
}

/// Reads the current App State from the settings location
/// Default location = /home/UHSER/.config/scout/config.json
pub fn read_filesapp_state() -> Result<FilesApp, std::io::Error> {
    let mut config = get_home_dir();
    config.push(".config");
    config.push("scout");
    config.push(CONFIG_NAME);
    match fs::metadata(config.clone()) {
        Ok(_) => match fs::read_to_string(config) {
            Ok(file_string) => {
                let app: FilesApp = serde_json::from_str(&file_string).unwrap();
                Ok(app)
            }
            Err(e) => {
                println!("Could not read: {:?}", e);
                return Err(e);
            }
        },
        Err(e) => Err(e),
    }
}

pub fn save_filesapp_state(app: &mut FilesApp) {
    let mut config_path = get_home_dir();
    config_path.push(".config");
    config_path.push("scout");
    match fs::metadata(config_path.clone()) {
        Ok(_) => (),
        Err(_) => match fs::create_dir(config_path.clone()) {
            Ok(_) => {}
            Err(e) => {
                println!("Can't create confg dir: {:?}", e)
            }
        },
    }

    config_path.push(CONFIG_NAME);

    let json_data = json!({
    "seperator": app.seperator,
    "selected_element_index": 0,
    "selected_element": ItemElement{
        name: "".to_string(),
        path: "".into(),
        _dir: false,
    },

    "files": vec![ItemElement{
        name: "".to_string(),
        path: "".into(),
        _dir: false,
    }],
    "preview": app.preview,

    "target": "".to_string(),

    "new_file_name": app.new_file_name,
    "search_string": app.search_string,
    "hide_hidden_files": app.hide_hidden_files,

    "image_formats": app.image_formats,
    "content": app.content,
    "history": app.history,

    "empty": app.empty,
    "current_path": app.current_path,
    "last_path": app.last_path,

    "app_mode": app.app_mode,

    "keybinds": app.keybinds,

    "debug": app.debug,
    "setting": app.setting,

    "double_g": app.double_g,
    "counter": app.counter,
    });

    match fs::write(config_path, json_data.to_string()) {
        Ok(_) => {
            println!("Saved app state")
        }
        Err(e) => {
            println!("Could not Save app State: {:?}", e)
        }
    }
}
