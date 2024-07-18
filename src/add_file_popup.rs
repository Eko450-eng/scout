use egui::{Context, Pos2, Vec2};
use struct_iterable::Iterable;

use crate::{
    file_man::{add_file, refresh_folder, rename_file},
    types::Modes,
    FilesApp,
};

/// Gives Position and Size for a centered Poup
/// # Examples 
/// ```
/// let (pos, size) = center_popup(ctx)
/// ```
pub fn center_popup(ctx: Context) -> (Pos2, egui::Vec2) {
    let vh = ctx.input(|i| i.screen_rect().y_range());
    let vw = ctx.input(|i| i.screen_rect().x_range());

    let size: Vec2 = Vec2 {
        x: vh.max * 0.30,
        y: vw.max * 0.30,
    };

    let pos: Pos2 = Pos2 {
        x: (vw.max - size.x) / 2.0,
        y: (vh.max - size.y) / 2.0,
    };

    return (pos, size);
}

pub fn add_file_popup(ctx: Context, app: &mut FilesApp) -> Option<egui::InnerResponse<Option<()>>> {
    let (pos, size) = center_popup(ctx.clone());

    let window = egui::Window::new("Create a file")
        .default_size(size)
        .default_open(true)
        .default_pos(pos);

    match app.app_mode {
        Modes::Creation => window.show(&ctx, |ui| {
            ui.text_edit_singleline(&mut app.new_file_name)
                .request_focus();
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let mut path = app.selected_element.clone().path;
                path.pop();

                let name =
                    app.current_path.to_string_lossy().to_string() + "/" + &app.new_file_name;

                add_file(path, name);
                refresh_folder(
                    app,
                    app.current_path.clone(),
                    app.hide_hidden_files,
                    app.search_string.clone(),
                );
            } else if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                app.app_mode = Modes::Action
            }
        }),
        _ => return None,
    }
}

pub fn move_file_popup(
    ctx: Context,
    app: &mut FilesApp,
) -> Option<egui::InnerResponse<Option<()>>> {
    let (pos, size) = center_popup(ctx.clone());

    let window = egui::Window::new("Rename")
        .default_size(size)
        .default_open(true)
        .default_pos(pos);

    match app.app_mode {
        Modes::Renaming => window.show(&ctx, |ui| {
            ui.text_edit_singleline(&mut app.target).request_focus();
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let original_file = app.selected_element.clone().path;
                let mut original_file_parent = app.selected_element.clone().path;
                original_file_parent.pop();

                rename_file(app, original_file);
                refresh_folder(
                    app,
                    app.current_path.clone(),
                    app.hide_hidden_files,
                    app.search_string.clone(),
                );
            } else if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                app.app_mode = Modes::Action
            }
        }),
        _ => return None,
    }
}

pub fn setings_popup(
    ctx: Context,
    app: &mut FilesApp,
) -> Option<egui::InnerResponse<Option<egui::InnerResponse<()>>>> {
    let (pos, size) = center_popup(ctx.clone());

    let window = egui::Window::new("Settings")
        .default_size(size)
        .default_open(true)
        .default_pos(pos);

    if app.setting {
        window.show(&ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Home Directory");
                });
                ui.horizontal(|ui| ui.checkbox(&mut app.debug, "Debugging"));
                ui.horizontal(|ui| ui.checkbox(&mut app.preview, "Live Preview"));
                ui.horizontal(|ui| ui.checkbox(&mut app.hide_hidden_files, "Show hidden Files"));
                ui.horizontal(|ui| {
                    ui.label("Image Formats");
                    for i in app.image_formats.clone() {
                        ui.label(i);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Keybinds");
                    for i in app.keybinds.iter() {
                        ui.label(i.0);
                    }
                });
            })
        })
    } else {
        return None;
    }
}
