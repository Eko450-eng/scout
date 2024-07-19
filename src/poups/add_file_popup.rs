use egui::{Context, Pos2, Vec2};

use crate::{
    scout_utils::file_man::{add_file, refresh_folder, rename_file},
    types::{FilesApp, Modes},
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

                let name = app.current_path.to_string_lossy().to_string()
                    + &app.seperator
                    + &app.new_file_name;

                add_file(path, name, app.seperator.clone());
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

