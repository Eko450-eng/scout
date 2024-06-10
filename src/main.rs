mod file_move;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs, usize};

use file_move::{
    create_file, delete_file, get_current_folder, get_folders_list, get_root_dir_files, rename_file,
};
use slint::{Model, ModelRc, SharedString, StandardListViewItem, VecModel};

const _APP_ID: &str = "de.wipdesign.scout";
const _APP_NAME: &str = "Scout";
const _APP_VERSION: &str = "0.0.1";

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // Constants
    let window = AppWindow::new()?;
    let ui = window.as_weak().clone();
    let mainui = window.as_weak().clone();

    // Set defaults
    let mut history: Vec<PathBuf> = [].to_vec();

    let mut root = get_current_folder();
    let current_path = root.clone();

    root.pop();
    history.push(root);
    history.push(current_path);

    let st = history.last().unwrap();
    ui.unwrap().set_last_path(SharedString::from(
        st.clone().into_os_string().into_string().unwrap(),
    ));

    let parent_files_list =
        slint::ModelRc::new(VecModel::from(get_root_dir_files(history[0].clone())));
    let files_list = slint::ModelRc::new(VecModel::from(get_root_dir_files(history[1].clone())));

    let mut ii = vec![];

    for i in files_list.iter() {
        let it = StandardListViewItem::from(i.text);
        ii.push(it);
    }

    set_parent(ui.unwrap(), parent_files_list);
    set_child(ui.unwrap(), files_list.clone());

    let mut depth = 1;

    // Handle Interaction
    ui.unwrap().invoke_mainfocus(true);

    let keybinds: keybinds = keybinds {
        up: SharedString::from("k"),
        down: SharedString::from("j"),
        into: SharedString::from("l"),
        outof: SharedString::from("h"),
        find: SharedString::from("f"),
        quit: SharedString::from("q"),
        esc: SharedString::from("\u{1b}"),
        delete: SharedString::from("d"),
        add: SharedString::from("a"),
        search: SharedString::from("p"),
        moving: SharedString::from("m"),
    };

    ui.unwrap().set_keybind(keybinds.clone());

    mainui.unwrap().on_refresh(move || {
        let h = mainui.unwrap().get_last_path();
        let files_list = slint::ModelRc::new(VecModel::from(get_root_dir_files(PathBuf::from(
            h.to_string(),
        ))));

        let mut ii = vec![];

        for i in files_list.iter() {
            let it = StandardListViewItem::from(i.text);
            ii.push(it);
        }
        set_child(mainui.unwrap(), files_list.clone());
    });

    let findui = window.as_weak().clone();
    findui.unwrap().on_find(move || {
        findui.unwrap().set_position(pos { x: 0, y: 0 });
        let parent_path = findui.unwrap().get_last_path();
        let pathbuf: PathBuf = parent_path.to_string().into();

        let mut ii = vec![];
        let current_dir = fs::read_dir(pathbuf).unwrap();

        let f = findui.unwrap().get_find_value().to_string();

        for name in current_dir {
            let name = name.unwrap().path();
            if name.to_str().is_some() {
                if name
                    .to_str()
                    .expect("Not a thing to add to find")
                    .contains(&f)
                {
                    if name.file_name().is_some() {
                        let shared_string = SharedString::from(
                            name.file_name().expect("No File Name").to_str().unwrap(),
                        );
                        let it = StandardListViewItem::from(shared_string);
                        ii.push(it);
                    }
                }
            }
        }

        let cfs = ModelRc::new(VecModel::from(ii.clone()));
        findui.unwrap().set_child_files_standard(cfs);
    });

    let deleteui = window.as_weak();
    deleteui
        .unwrap()
        .on_delete(move |value| delete_file(value.to_string().into()));

    let addui = window.as_weak();
    addui.unwrap().on_create(move |value| {
        create_file(
            addui.unwrap().get_last_path().to_string().into(),
            value.into(),
        )
    });

    let moveui = window.as_weak();
    moveui.unwrap().on_move(move |value| {
        let path = PathBuf::from(moveui.unwrap().get_last_path().to_string());
        let mut target = path.clone();
        let selection = moveui
            .unwrap()
            .get_child_files_standard()
            .row_data(moveui.unwrap().get_child_pos() as usize);

        target.push(selection.unwrap().text.to_string());
        rename_file(
            target,
            value.into(),
        )
    });

    ui.unwrap().on_key_presed(move |key_event| {
        let key = key_event.clone().text;
        if !ui.unwrap().get_find_box_focus() {
            if key == keybinds.moving {
                let mut path = PathBuf::from(ui.unwrap().get_last_path().to_string());
                let target = path.clone();
                let selection = ui
                    .unwrap()
                    .get_child_files_standard()
                    .row_data(ui.unwrap().get_child_pos() as usize);

                ui.unwrap()
                    .set_selected_file(selection.clone().unwrap().text);

                path.push(selection.unwrap().text.to_string());

                ui.unwrap()
                    .set_move_file_name(SharedString::from(target.to_str().unwrap()));

                ui.unwrap().set_move_file_visible(true);
                ui.unwrap().invoke_movefilefocus(true);
                ui.unwrap().invoke_mainfocus(false);
            } else if key == keybinds.esc {
                ui.unwrap().invoke_findboxfocus(false);
                ui.unwrap().invoke_mainfocus(true);
            } else if key == keybinds.delete {
                ui.unwrap().invoke_deletefocus(true);
                ui.unwrap().invoke_mainfocus(false);
                let data = ui
                    .unwrap()
                    .get_child_files_standard()
                    .row_data(ui.unwrap().get_child_pos() as usize)
                    .unwrap();

                let mut path: PathBuf = history.last().clone().unwrap().to_path_buf();
                path.push(data.text.to_string());
                let file_name = path.to_str();
                ui.unwrap()
                    .set_delete_file_name(SharedString::from(file_name.unwrap()));
                ui.unwrap().set_delete_file_visible(true);
            } else if key == keybinds.add {
                ui.unwrap().set_new_file_visible(true);
                ui.unwrap().invoke_newfilefocus(true);
                ui.unwrap().invoke_mainfocus(false);
            } else if key == keybinds.up {
                move_y(ui.unwrap(), "up".to_string());
                let viewport_y = ui.unwrap().get_child_viewport_y();
                if viewport_y < 0.0 {
                    ui.unwrap().invoke_scroll("up".into());
                }
            } else if key == keybinds.down {
                let visible_height = ui.unwrap().get_child_visible_height();
                let position = ui.unwrap().get_child_pos();

                if (position as f32 * 25.0) > visible_height / 2.0 {
                    ui.unwrap().invoke_scroll("down".into());
                }

                move_y(ui.unwrap(), "down".to_string())
            } else if key == keybinds.quit {
                set_child(ui.unwrap(), files_list.clone());
                ui.unwrap().set_position(pos { x: 0, y: 0 });
            } else if key == keybinds.find {
                ui.unwrap().invoke_findboxfocus(true);
                ui.unwrap().invoke_mainfocus(false);
            } else if key == keybinds.outof {
                if depth > 1 {
                    let (y, x, d) = move_out(ui.unwrap(), &mut history, depth);
                    let new_pos = pos { x, y };
                    ui.unwrap().set_position(new_pos);
                    depth = d;
                }
            } else if key == keybinds.into {
                let (y, x, d) = move_in(ui.unwrap(), &mut history, depth);
                let new_pos = pos { x, y };
                ui.unwrap().set_position(new_pos);
                depth = d;
            }
        } else if key == keybinds.esc {
            ui.unwrap().invoke_findboxfocus(false);
            ui.unwrap().invoke_mainfocus(true)
        } else {
            return;
        }
    });

    // Startup
    window.run()
}

fn set_parent(ui: AppWindow, files_list: ModelRc<StandardListViewItem>) {
    let mut ii = vec![];

    for i in files_list.iter() {
        let it = StandardListViewItem::from(i.text);
        ii.push(it);
    }
    let cfs = ModelRc::new(VecModel::from(ii.clone()));

    ui.set_files(cfs.clone());
}

fn set_child(ui: AppWindow, files_list: ModelRc<StandardListViewItem>) {
    let mut ii = vec![];

    for i in files_list.iter() {
        let it = StandardListViewItem::from(i.text);
        ii.push(it);
    }
    let cfs = ModelRc::new(VecModel::from(ii.clone()));

    ui.set_child_files_standard(cfs.clone());
}

fn move_y(ui: AppWindow, dir: String) {
    let mut current_position = ui.get_position().clone();

    if dir == "up" && ui.get_child_pos() > 0 {
        current_position.y -= 1;
    } else if dir == "down"
        && ((ui.get_child_pos() + 1) as usize) < ui.get_child_files_standard().iter().len()
    {
        current_position.y += 1;
    }
    ui.set_position(pos {
        x: current_position.x,
        y: current_position.y,
    })
}

fn move_in(ui: AppWindow, history: &mut Vec<PathBuf>, depth: i32) -> (i32, i32, i32) {
    let mut new_path: PathBuf = PathBuf::new();
    if history.last().is_some() {
        new_path = history.last().expect("No last Element").clone();
    }

    print!("1");
    let data = ui
        .get_child_files_standard()
        .row_data(ui.get_child_pos() as usize);
    let name = Some(data).unwrap_or_default();
    new_path.push(name.unwrap().text.to_string());

    print!("2");
    if new_path.is_dir() {
        print!("3");
        history.push(new_path);

        let mut child = history[0].clone();

        if history[history.len() - 1].exists() {
            child = history[history.len() - 1].clone();
        }

        let mut parent = history[0].clone();
        if history[history.len() - 2].exists() {
            parent = history[history.len() - 2].clone();
        }
        let parent_files_list = get_folders_list(parent);

        let files_list = get_folders_list(child.clone());

        set_parent(ui.clone_strong(), parent_files_list);
        set_child(ui.clone_strong(), files_list);
        let st = history.last().unwrap();
        ui.set_last_path(SharedString::from(
            st.clone().into_os_string().into_string().unwrap(),
        ));

        return (
            0,
            0,
            depth + 1,
        );
    } else if new_path.extension().is_some() {
        print!("4");
        let new_path_ext = new_path
            .extension()
            .expect(&format!("Failed to read extension of: {:?}", new_path));
        let ext = new_path_ext;
        let ext_str = ext.to_str();
        match ext_str {
            Some("lua") => {
                Command::new("wezterm")
                    .arg("start")
                    .arg("nvim")
                    .arg(&new_path)
                    .status()
                    .expect("Failed to open");
            }
            _ => {
                if fs::read_to_string(new_path.clone()).is_ok() {
                    print!("Okay");
                    let c = fs::read_to_string(new_path.clone()).expect("Failed to Read file");

                    let content = SharedString::from(c);

                    ui.set_content_of_file(content)
                }
                print!("HI")
            }
        }
        return (ui.get_position().y, ui.get_position().x, depth);
    } else {
        if fs::read_to_string(new_path.clone()).is_ok() {
            print!("Okay");
            let c = fs::read_to_string(new_path.clone()).expect("Failed to Read file");

            let content = SharedString::from(c);

            ui.set_content_of_file(content);
        }
        return (0, 0, 0);
    }
}

fn move_out(ui: AppWindow, history: &mut Vec<PathBuf>, depth: i32) -> (i32, i32, i32) {
    let mut new_path = history[depth as usize].clone();

    new_path.pop();

    if new_path.is_dir() {
        if history.len() > 1 {
            history.pop();
        }

        let mut child = history[0].clone();

        let mut parent = history[0].clone();
        if history[history.len() - 2].exists() {
            parent = history[history.len() - 2].clone();
        }
        let parent_files_list = get_folders_list(parent);

        if history[history.len() - 1].exists() {
            child = history[history.len() - 1].clone();
        }

        let files_list = get_folders_list(child);

        set_parent(ui.clone_strong(), parent_files_list);
        set_child(ui, files_list);

        // ui.set_files(parent_files_list);

        return (0, 0, depth - 1);
    } else {
        return (ui.get_position().y, ui.get_position().x, depth);
    }
}
