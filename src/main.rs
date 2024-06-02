use std::cell::Cell;
use std::fs;
use std::path::Path;

use gtk::{prelude::*, Box, ListBox, Orientation};

use gtk::Label;

use gtk::{gdk, glib, Application, ApplicationWindow};

const APP_ID: &str = "de.wipdesign.scout";
const APP_NAME: &str = "Scout";
const _APP_VERSION: &str = "0.0.1";

fn get_current_folder() -> &'static Path {
    if cfg!(target_os = "windows") {
        Path::new("C:\\")
    } else {
        Path::new("/home/eko/.config/nvim")
    }
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let position = Cell::new(0);

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

    // Boxes
    let wrapper_box = Box::new(Orientation::Horizontal, 5);
    let btn_box = Box::new(Orientation::Vertical, 5);
    let parent_box = ListBox::new();

    // Button
    let down_btn = gtk::Button::builder().label("Down").build();

    let up_btn = gtk::Button::builder().label("Up").build();

    down_btn.connect_clicked(move |parent_box| {
        println!("Pressed Down");
        parent_box.emit_move_focus(gtk::DirectionType::Down)
    });

    up_btn.connect_clicked(move |parent_box| {
        println!("Pressed Up");
        parent_box.emit_move_focus(gtk::DirectionType::Up)
    });

    btn_box.append(&down_btn);
    btn_box.append(&up_btn);

    wrapper_box.append(&btn_box);
    wrapper_box.append(&parent_box);

    // Window
    let window = ApplicationWindow::builder()
        .title(APP_NAME)
        .application(app)
        .child(&wrapper_box)
        .build();

    window.set_default_size(300, 300);

    for (_i, name) in files.iter().enumerate() {
        let text_field = Label::builder().label(format!("{:?}", name)).build();

        parent_box.append(&text_field)
    }

    let event_controller = gtk::EventControllerKey::new();

    parent_box.connect_move_focus(|one, two|{
        println!("Focus {:?} {:?}", one, two)
    });
    
    parent_box.connect_move_cursor(|one, two, three, four, five|{
        println!("Cursor {:?} {:?} {:?} {:?} {:?}", one, two, three, four, five)
    });

    event_controller.connect_key_pressed(move |_, key, _, _| {
        match key {
            gdk::Key::j => {
                println!("Pressed Down {:?}", position.get());
                parent_box.emit_move_cursor(gtk::MovementStep::DisplayLines, 1, false, false)
            }
            gdk::Key::k => {
                println!("Pressed Up {:?}", position.get());
                parent_box.emit_move_cursor(gtk::MovementStep::DisplayLines, -1, false, false)
            }
            _ => {}
        };

        glib::Propagation::Proceed
    });

    window.add_controller(event_controller);

    window.present();
}
