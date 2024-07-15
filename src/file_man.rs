use std::{fs::{self, read_dir, File}, io::{self, BufReader, Read, Write}, path::{Path, PathBuf}};

use crate::ItemElement;

pub fn get_root_dir_files(dir: PathBuf) -> Vec<ItemElement> {
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
    file.retain(|item| !item.name.starts_with("."));

    return file;
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

pub fn get_content(target: PathBuf) -> String {
    let mut files_list: Vec<String> = vec![];
    if target.is_file() {
        match is_utf8(target.clone()) {
            Ok(true) => fs::read_to_string(target.clone()).expect("Failed to Read file"),
            Ok(false) => "Hi".to_string(),
            Err(_) => "Error".to_string(),
        }
    } else {
        let items = get_root_dir_files(target);
        for i in items {
            files_list.push(i.name)
        }
        return files_list.join("\n");
    }
}

pub fn rename_file(file: PathBuf, target_string: String) {
    let original_file_name = file.clone();

    let mut original_file_parent_location = file.clone();
    original_file_parent_location.pop();

    let mut new_file_name = original_file_parent_location.clone();
    new_file_name.push(target_string);

    println!("Original name: {:?}", original_file_name);
    println!("Original location: {:?}", original_file_parent_location);
    println!("New name: {:?}", new_file_name);

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
