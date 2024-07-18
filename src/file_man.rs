use std::{
    ffi::OsString,
    fs::{self, read_dir, File},
    io::{self, BufReader, Read, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{types::FilesApp, ItemElement};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub enum FileContentType {
    Dir,
    Txt,
    Image,
    Binary,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileContent {
    pub content: String,
    pub file_type: FileContentType,
    pub read: bool,
}




// TODO: Make this a setting
const MAX_SIZE: u64 = 1000;

/// Gives back all files inside given directory
/// Search Term when starting with ! will be translated to regex
/// # Examples
/// ```rust
/// let app.files = get_root_dir_files("home/user/Desktop", true, "search term".to_string());
/// ```
pub fn get_root_dir_files(
    dir: PathBuf,
    hide_hidden_files: bool,
    search_term: String,
) -> Vec<ItemElement> {
    let mut file: Vec<ItemElement> = read_dir(dir)
        .expect("Failed to read directory")
        .enumerate()
        .filter_map(|(_index, entry)| {
            entry.ok().and_then(|en| {
                Some(ItemElement {
                    name: en.file_name().to_str().unwrap().to_string(),
                    path: en.path(),
                    _dir: en.path().is_dir(),
                })
            })
        })
        .collect();

    file.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    if hide_hidden_files {
        file.retain(|item| !item.name.starts_with("."));
    }
    if search_term != "" {
        if search_term.starts_with("!") {
            let ss = &search_term[1..];
            file.retain(|item| {
                regex::Regex::new(&ss)
                    .expect("Can't create the regex")
                    .is_match(&item.name)
            });
        } else {
            if contains_uppercase(&search_term) {
                file.retain(|item| item.name.contains(&search_term));
            } else {
                file.retain(|item| item.name.to_lowercase().contains(&search_term));
            }
        }
    }

    return file;
}

fn contains_uppercase(input: &str) -> bool {
    for c in input.chars() {
        if c.is_uppercase() {
            return true;
        }
    }
    false
}

pub fn is_utf8<P: AsRef<Path>>(path: P) -> io::Result<bool> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1024];

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        if let Err(_) = std::str::from_utf8(&buffer[..bytes_read]) {
            return Ok(false);
        }
    }

    Ok(true)
}

pub fn get_content(target: PathBuf) -> FileContent {
    let mut files_list: Vec<String> = vec![];
    let mut file_size: u64 = 0;

    let mut out = FileContent {
        content: "".to_string(),
        file_type: FileContentType::Dir,
        read: true,
    };

    if target.clone().is_file() {
        out.file_type = FileContentType::Txt;

        let image_ext: Vec<String> = vec![
            "png".to_string(),
            "jpg".to_string(),
            "jpeg".to_string(),
            "JPEG".to_string(),
            "JPG".to_string(),
        ];
        let mut image_ext_osstring: Vec<OsString> = vec![];
        for i in image_ext {
            image_ext_osstring.push(OsString::from(i))
        }

        let ext = target.extension();
        if ext.is_some() {
            if image_ext_osstring.contains(&ext.unwrap().to_os_string()) {
                out.file_type = FileContentType::Image
            }
        }

        if let Ok(metadata) = fs::metadata(target.clone()) {
            let m = metadata.len();
            file_size = m;
        }

        match is_utf8(target.clone()) {
            Ok(true) => {
                if file_size > MAX_SIZE {
                    out.read = false;
                    out
                } else {
                    out.content = fs::read_to_string(target.clone()).expect("Failed to Read file");
                    out
                }
            }
            Ok(false) => out,
            Err(_) => out,
        }
    } else {
        let items = get_root_dir_files(target, true, "".to_string());
        for i in items {
            files_list.push(i.name)
        }
        out.file_type = FileContentType::Dir;
        out.content = files_list.join("\n");
        return out;
    }
}

pub fn rename_file(app: &mut FilesApp, file: PathBuf) {
    let original_file_name = file.clone();

    let mut original_file_parent_location = file.clone();
    original_file_parent_location.pop();

    let new_file_name = PathBuf::from(app.target.clone());

    match fs::rename(original_file_name.clone(), new_file_name.clone()) {
        Ok(_) => return,
        Err(err) => {
            println!(
                "Could not Rename / Move from {:?} to {:?} because: {:?}",
                original_file_name, new_file_name, err
            )
        }
    }
}

pub fn add_file(target: PathBuf, name: String) {
    let content: &[u8] = b"";
    if target.is_dir() {
        let mut target_item = target.clone();
        target_item.push(name.to_string());

        // Check wether it is a path or file name
        if target_item
            .to_str()
            .expect("Target item can't be converted to String")
            .contains("/")
        {
            let mut final_file_name: PathBuf = PathBuf::new();
            let mut create_file = false;

            // Check if last element is a file or still a path
            if !name.ends_with("/") {
                final_file_name = name
                    .clone()
                    .split("/")
                    .last()
                    .expect("Can't split into file name")
                    .into();
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
                        file.expect("Cannot open file to write to it")
                            .write_all(content)
                            .expect("Can't write to file");
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
                file.expect("Can't open file")
                    .write_all(content)
                    .expect("Can't write content to file");
            } else {
                println!(
                    "Failed to Create file at {:?}\nBecause: {:?}",
                    target_item, file
                );
            }
        }
    } else {
        println!("Target is not a dir")
    }
}

pub fn delete_file(app: &mut FilesApp) {
    let file_path = app.selected_element.path.clone();

    if file_path.is_dir() {
        match fs::remove_dir_all(file_path.clone()) {
            Ok(_) => {
                refresh_folder(
                    app,
                    app.current_path.clone(),
                    app.hide_hidden_files,
                    app.search_string.clone(),
                );
            }
            Err(err) => println!("Could not delete: {:?} \nBecause {:?}", file_path, err),
        }
    } else if file_path.is_file() {
        match fs::remove_file(file_path.clone()) {
            Ok(_) => {
                refresh_folder(
                    app,
                    app.current_path.clone(),
                    app.hide_hidden_files,
                    app.search_string.clone(),
                );
            }
            Err(err) => println!("Could not delete: {:?} \nBecause {:?}", file_path, err),
        }
    } else {
        println!("Could not delete: {:?}", file_path);
    }
}

pub fn refresh_folder(
    app: &mut FilesApp,
    dir: PathBuf,
    hide_hidden_files: bool,
    search_string: String,
) {
    app.files = get_root_dir_files(dir, hide_hidden_files, search_string)
}

pub fn save_to_file(app: &mut FilesApp) {
    match fs::write(
        app.selected_element.path.clone(),
        app.content.content.clone(),
    ) {
        Ok(_) => {}
        Err(e) => {
            println!("Saving failed: {:?}", e)
        }
    }
}
