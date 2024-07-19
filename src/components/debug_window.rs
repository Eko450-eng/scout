use egui::{Context, Pos2, Vec2};

use crate::{scout_utils::file_man::FileContentType, FilesApp};

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
                    FileContentType::Dir => ui.label("Dir"),
                    FileContentType::Txt => ui.label("Txt"),
                    FileContentType::Image => ui.label("Img"),
                    FileContentType::Binary => ui.label("Bin"),
                }
            });
        })
    } else {
        return None;
    }
}
