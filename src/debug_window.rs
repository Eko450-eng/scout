use egui::{Context, Pos2, Vec2};

use crate::{file_man::add_file, types::Modes, FilesApp};

pub fn debug_window(ctx: Context, app: &mut FilesApp) -> Option<egui::InnerResponse<Option<()>>> {
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

    let window = egui::Window::new("Debug window")
        .default_size(size)
        .default_open(true)
        .default_pos(pos);

}
