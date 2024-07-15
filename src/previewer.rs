use egui::{text_edit::TextEditOutput, Context, ScrollArea, Ui};
use egui_code_editor::{CodeEditor, Syntax};

use crate::{file_man::get_root_dir_files, types::FilesApp};

pub fn show_preview(app: &mut FilesApp, ui: &mut Ui) -> egui::InnerResponse<TextEditOutput> {
    ui.vertical(|ui| {
        CodeEditor::default()
            .id_source("code_editor")
            .with_rows(12)
            .with_fontsize(12.0)
            .with_syntax(Syntax::default())
            .with_numlines(false)
            .show(ui, &mut app.content.content)
    })
}

pub fn show_dir(app: &mut FilesApp, ui: &mut Ui) {
    let content = app.content.content.clone();
    //let count = content.chars().filter(|&c| c == '\n').count();

    ScrollArea::vertical().show(ui, |ui| {
        ui.colored_label(egui::Color32::LIGHT_BLUE, app.content.content.clone());
    });
}

pub fn show_image(ctx: Context, app: &mut FilesApp, ui: &mut Ui) {
    // let uri = "file://".to_string() + &app.selected_element.path.to_string_lossy().to_string();
    let image = image::io::Reader::open(app.selected_element.clone().path)
        .unwrap()
        .decode()
        .unwrap();
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    let img = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

    let texture = ctx.load_texture("img", img, egui::TextureOptions::default());

    ui.image((texture.id(), texture.size_vec2()));
}
