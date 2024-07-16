#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod add_file_popup;
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
use eframe::{egui, App};
use egui::{Align, Layout};
use key_actions::{
    handle_key_action, handle_key_creation, handle_key_delete, handle_key_editing,
    handle_key_search,
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
            search_file_popup(ctx.clone(), self);

            match self.app_mode {
                Modes::Action => mode_display = "Action",
                Modes::Editing => mode_display = "Editing",
                Modes::Creation => mode_display = "Creation",
                Modes::Search => mode_display = "Search",
                Modes::Renaming => mode_display = "Renaming",
                Modes::Deletion => mode_display = "Deletion",
                Modes::Setting => mode_display = "Settings",
            }

            navigation_bar(ui, self);

            // Key events
            match self.app_mode {
                Modes::Action => handle_key_action(self, ui),
                Modes::Creation => handle_key_creation(self, ui),
                Modes::Search => handle_key_search(self, ui),
                Modes::Deletion => handle_key_delete(self, ui),
                Modes::Editing => handle_key_editing(self, ui),
                _ => (),
            };

            let vh = ctx.input(|i| i.screen_rect().y_range());
            ui.horizontal(|ui| {
                ui.set_height(vh.max - 100.0);
                ui.vertical(|ui| {
                    ui.set_height(vh.max - 200.0);

                    main_view(ui, self);
                });

                ui.vertical(|ui| {
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
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
                        ui.with_layout(Layout::bottom_up(Align::RIGHT), |ui| {
                            if matches!(self.app_mode, Modes::Deletion) {
                                ui.colored_label(
                                    egui::Color32::RED,
                                    "Are you sure you want to delete this? (Y | N)",
                                );
                            }
                            ui.colored_label(egui::Color32::LIGHT_BLUE, mode_display);
                        })
                    })
                });
            });
        });
    }
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let config = PathBuf::from("/home/eko/.config/scout/config.json");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([300.0, 200.0]),
        ..Default::default()
    };

    match fs::metadata(config.clone()) {
        Ok(_) => {
            let config_content = fs::read_to_string(config);
            println!("Config was read: {}", config_content.unwrap());
        }
        Err(_) => (),
    }

    eframe::run_native(
        "Scout",
        options,
        Box::new(|_cc| Ok(Box::new(FilesApp::default()))),
    )
}
