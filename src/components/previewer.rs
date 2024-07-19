use egui::{text_edit::TextEditOutput, Align, Context, Layout, Ui};
use egui_code_editor::{CodeEditor, Syntax};
use image::{imageops, GenericImageView};

use crate::types::FilesApp;

/// Shows A editable Code Editor preview with the app.content.content and writes it to the
/// currently selected files
pub fn show_preview(app: &mut FilesApp, ui: &mut Ui) -> egui::InnerResponse<TextEditOutput> {
    ui.vertical(|ui| {
        CodeEditor::default()
            .id_source("code_editor")
            .with_rows(12)
            .with_fontsize(12.0)
            .with_syntax(Syntax::default())
            .with_syntax(Syntax::shell())
            .with_syntax(Syntax::sql())
            .with_syntax(Syntax::rust())
            .with_numlines(true)
            .show(ui, &mut app.content.content)
    })
}

/// Shows an overview of the Elements inside the Directoy
pub fn show_dir(app: &mut FilesApp, ui: &mut Ui) -> egui::InnerResponse<()> {
    ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
        ui.colored_label(egui::Color32::LIGHT_BLUE, app.content.content.clone());
    })
}

fn load_image(app: &mut FilesApp) -> Result<image::DynamicImage, image::ImageError> {
    let image =
        image::io::Reader::open(app.selected_element.clone().path).expect("Error Decoding Image");

    match image.decode() {
        Ok(image) => {
            let target_height = 400;
            let (orig_width, orig_height) = image.dimensions();
            let aspect_ratio = orig_width as f32 / orig_height as f32;
            let target_width = (target_height as f32 * aspect_ratio).round() as u32;

            let resized_image =
                image.resize(target_width, target_height, imageops::FilterType::Nearest);
            Ok(resized_image)
        }
        Err(e) => {
            println!("Could not load because: {:?}", e);
            Err(e)
        }
    }
}

/// Shows the Image scaled down
/// WARN: In debugging mode this will be very slow apparently but super fast in --release
pub fn show_image(ctx: Context, app: &mut FilesApp, ui: &mut Ui) {
    if !app.preview {
        return;
    }

    match load_image(app) {
        Ok(image) => {
            let size = [image.width() as _, image.height() as _];

            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();

            let img = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

            let texture = ctx.load_texture("img", img, egui::TextureOptions::default());

            ui.image((texture.id(), texture.size_vec2()));
        }
        Err(e) => {
            println!("error -- {:?}", e)
        }
    }
}
