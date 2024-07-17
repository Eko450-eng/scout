#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod add_file_popup;
mod debug_window;
mod file_man;
mod key_actions;
mod main_view;
mod movement_actions;
mod navigation_bar;
mod previewer;
mod search_file_popup;
mod types;
mod utils;

use std::{fs, path::PathBuf};

use add_file_popup::{add_file_popup, move_file_popup, setings_popup};
use debug_window::debug_window;
use eframe::{egui, App};
use key_actions::{
    handle_key_action, handle_key_creation, handle_key_delete, handle_key_editing,
    handle_key_search, read_filesapp_state,
};
use main_view::main_view;
use navigation_bar::navigation_bar;
use previewer::{show_dir, show_image, show_preview};
use search_file_popup::search_file_popup;
use types::{FilesApp, ItemElement, Modes};

impl App for FilesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            let mode_display;
            add_file_popup(ctx.clone(), self);
            move_file_popup(ctx.clone(), self);
            setings_popup(ctx.clone(), self);
            debug_window(ctx.clone(), self);
            search_file_popup(ctx.clone(), self);

            if self.double_g && self.counter < 50 {
                self.counter = self.counter + 1;
            } else {
                self.double_g = false;
                self.counter = 0;
            }

            match self.app_mode {
                Modes::Action => mode_display = "Action",
                Modes::Editing => mode_display = "Editing",
                Modes::Creation => mode_display = "Creation",
                Modes::Search => mode_display = "Search",
                Modes::Renaming => mode_display = "Renaming",
                Modes::Deletion => mode_display = "Deletion",
                Modes::NonAction => mode_display = "Settings",
            }

            // Key events
            match self.app_mode {
                Modes::Action => handle_key_action(self, ui),
                Modes::Creation => handle_key_creation(self, ui),
                Modes::Search => handle_key_search(self, ui),
                Modes::Deletion => handle_key_delete(self, ui),
                Modes::Editing => handle_key_editing(self, ui),
                _ => (),
            };

            egui::TopBottomPanel::top("menu_bar").show_inside(ui, |ui| {
                navigation_bar(ui, self);
            });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                let height = ui.available_height();
                ui.horizontal(|ui| {
                    ui.set_height(height);
                    main_view(ui, self);

                    if self.preview {
                        match self.content.file_type {
                            file_man::FileContentType::Dir => {
                                show_dir(self, ui);
                            }
                            file_man::FileContentType::Txt => {
                                show_preview(self, ui);
                            }
                            file_man::FileContentType::Image => {
                                show_image(ctx.clone(), self, ui);
                            }
                            _ => {
                                if !self.content.read {
                                    ui.label("Too big to Preview");
                                }
                            }
                        }
                    }
                });
            });

            egui::TopBottomPanel::bottom("status_bar")
                .resizable(false)
                .show_inside(ui, |ui| {
                    ui.colored_label(egui::Color32::LIGHT_BLUE, mode_display);
                    if matches!(self.app_mode, Modes::Deletion) {
                        ui.colored_label(
                            egui::Color32::RED,
                            "Are you sure you want to delete this? (Y | N)",
                        );
                    }
                })
        });
    }
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let mut config: FilesApp = FilesApp::default();
    match read_filesapp_state() {
        Ok(app) => config = app,
        Err(_) => {}
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id("Scout")
            .with_inner_size([600.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Scout",
        options,
        Box::new(|_cc| Ok(Box::new(config))),
    )
}
