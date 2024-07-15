use egui::{Context, Pos2, Vec2};

use crate::{file_man::add_file, types::Modes, FilesApp};

pub fn add_file_popup(ctx: Context, app: &mut FilesApp) -> Option<egui::InnerResponse<Option<()>>> {
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
            } else if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                app.app_mode = Modes::Action
            }
        }),
        _ => return None,
    }
}
