use std::fs;
use std::path::Path;

use gtk::{gdk, Label};
use std::cell;

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box};

const APP_ID: &str = "de.wipdesign.scout";
const APP_NAME: &str = " ----Scout";
const _APP_VERSION: &str = "0.0.1";

fn get_current_folder() -> &'static Path {
    if cfg!(target_os = "windows") {
        Path::new("C:\\")
    } else {
        Path::new("/home/eko/")
    }
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let position = cell::Cell::new(0);

    let files = fs::read_dir(get_current_folder())
        .expect("Fail")
        .filter_map(|paths| {
            paths.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>();


    let parent_box = Box::new(gtk::Orientation::Vertical, 5);

    let window = ApplicationWindow::builder()
        .title(APP_NAME)
        .application(app)
        .child(&parent_box)
        .build();

    window.set_default_size(300, 300);

    for (i, name) in files.iter().enumerate() {
        let addition = if position.get() == i{
          format!("{:?} {:?}", name, "<-")
        }else{
          format!("{:?}", name)
        };
        
        let text_field = Label::builder()
            .label(addition)
            .build();

        parent_box.append(&text_field)
    }

    let event_controller = gtk::EventControllerKey::new();

    event_controller.connect_key_pressed(move |_, key, _, _| {
        match key {
            gdk::Key::h => {
                println!("Left")
            }
            gdk::Key::j => {
                println!("Total {:?}, \nCurrent {:?}", files.len(), position.get());
                if position.get() != 0 {
                    position.set(position.get() + 1);
                }
            }
            gdk::Key::k => {
                println!("Total {:?}, \nCurrent {:?}", files.len(), position.get());
                if position.get() < files.len() {
                    position.set(position.get() - 1);
                }
            }
            gdk::Key::l => {
                println!("Right")
            }
            _ => {}
        }

        glib::Propagation::Proceed
    });

    window.add_controller(event_controller);

    window.present();
}
