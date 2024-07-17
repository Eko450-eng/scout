use egui::{Context, Pos2, Vec2};

use crate::FilesApp;

pub fn debug_window(ctx: Context, app: &mut FilesApp) -> Option<egui::InnerResponse<Option<()>>> {
    let back_path = app.current_path.clone();

    let vh = ctx.input(|i| i.screen_rect().y_range());
    let vw = ctx.input(|i| i.screen_rect().x_range());

    let size: Vec2 = Vec2 {
        x: vh.max * 0.30,
        y: vw.max * 0.30,
    };

    let pos: Pos2 = Pos2 {
        x: vw.max,
        y: vh.min,
    };

    let window = egui::Window::new("Debug window")
        .default_size(size)
        .default_open(true)
        .default_pos(pos)
        .current_pos(pos);

    if app.debug {
        window.show(&ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Back Path: ");
                ui.label(back_path.to_string_lossy().to_string())
            });

            ui.horizontal(|ui| {
                ui.label("Last Path: ");
                ui.label(app.last_path.to_string_lossy().to_string())
            });

            ui.horizontal(|ui| {
                ui.label("Current Path: ");
                ui.label(app.current_path.to_string_lossy().to_string());
            });

            ui.horizontal(|ui| {
                ui.label("Selected: ");
                ui.label(app.selected_element.path.to_string_lossy().to_string());
                ui.label(app.selected_element.name.clone());
                match app.content.file_type {
                    crate::file_man::FileContentType::Dir => ui.label("Dir"),
                    crate::file_man::FileContentType::Txt => ui.label("Txt"),
                    crate::file_man::FileContentType::Image => ui.label("Img"),
                    crate::file_man::FileContentType::Binary => ui.label("Bin"),
                }
            });
        })
    } else {
        return None;
    }
}
