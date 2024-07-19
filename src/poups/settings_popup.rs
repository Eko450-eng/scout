use egui::Context;
use struct_iterable::Iterable;

use crate::types::FilesApp;

use super::add_file_popup::center_popup;

pub fn settings_popup(
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
