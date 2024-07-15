use std::{
    ffi::OsStr,
    fs::{self, read_dir, File},
    io::{self, BufReader, Read, Write},
    path::{Path, PathBuf},
};

use crate::ItemElement;

const MAX_SIZE: u64 = 1000;

pub fn get_root_dir_files(
    dir: PathBuf,
    hide_hidden_files: bool,
    search_string: String,
) -> Vec<ItemElement> {
    let mut file: Vec<ItemElement> = read_dir(dir)
        .expect("Fail")
        .enumerate()
        .filter_map(|(_index, entry)| {
            entry.ok().and_then(|en| {
                Some(ItemElement {
                    name: en.file_name().into_string().expect("No file name found"),
                    path: en.path(),
                    dir: en.path().is_dir(),
                })
            })
        })
        .collect();

    file.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    if hide_hidden_files {
        file.retain(|item| !item.name.starts_with("."));
    }
    if search_string != "" {
        if search_string.starts_with("!") {
            let ss = &search_string[1..];
            file.retain(|item| regex::Regex::new(&ss).unwrap().is_match(&item.name));
        } else {
            if contains_uppercase(&search_string) {
                file.retain(|item| item.name.contains(&search_string));
            } else {
                file.retain(|item| item.name.to_lowercase().contains(&search_string));
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

pub enum FileContentType {
    Dir,
    Txt,
    Image,
    Binary,
}

pub struct FileContent {
    pub content: String,
    pub file_type: FileContentType,
    pub read: bool,
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

        if target.extension() == Some(OsStr::new("png")) {
            out.file_type = FileContentType::Image
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

pub fn rename_file(file: PathBuf, target_string: String) {
    let original_file_name = file.clone();

    let mut original_file_parent_location = file.clone();
    original_file_parent_location.pop();

    let mut new_file_name = original_file_parent_location.clone();
    new_file_name.push(target_string);

    match fs::rename(original_file_name.clone(), new_file_name.clone()) {
        Ok(_) => {
            println!("Moved from {:?} to {:?}", original_file_name, new_file_name)
        }
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
