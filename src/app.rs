#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[cfg(target_arch = "wasm32")]
use rfd::AsyncFileDialog;

#[derive(Default)]
pub struct FileApp {
    names: Arc<Mutex<Vec<String>>>,
    picked_path: Option<Vec<String>>,
}

impl eframe::App for FileApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            egui::trace!(ui);
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ui, frame);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Pick a file!");

            #[cfg(not(target_arch = "wasm32"))]
            if let Some(picked_path) = &self.picked_path {
                for str in picked_path {
                    ui.horizontal(|ui| {
                        ui.label("Picked file:");
                        ui.monospace(str);
                    });
                }
            }
            #[cfg(target_arch = "wasm32")]
            {
                let names = &self.names.lock().unwrap().to_vec();
                for str in names {
                    ui.horizontal(|ui| {
                        ui.label("Picked file:");
                        ui.monospace(str);
                    });
                }
            }
        });
    }
}
impl FileApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        FileApp {
            names: Arc::new(Mutex::new(Default::default())),
            picked_path: None,
        }
    }
    fn bar_contents(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::widgets::global_dark_light_mode_switch(ui);

        ui.separator();

        if ui.button("Open file...").clicked() {
            if let Some(path) = self.fileclick() {
                let mut files: Vec<String> = Default::default();
                for file in path {
                    files.push(file.to_str().unwrap().to_string());
                }
                self.picked_path = Some(files);
            };
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn fileclick(&mut self) -> Option<Vec<PathBuf>> {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("MOCs", &["fits", "ascii", "json", "txt"])
            .pick_files()
        {
            return Some(path);
        } else {
            return None;
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn fileclick(&mut self) -> Option<Vec<PathBuf>> {
        let task = AsyncFileDialog::new()
            .add_filter("MOCs", &["fits", "ascii", "json", "txt"])
            .pick_files();
        let names_cpy = self.names.clone();

        Self::execute(async move {
            let file = task.await;
            let mut names: Vec<String> = Default::default();

            if let Some(file) = file {
                // If you care about wasm support you just read() the file
                for path in file {
                    let file_name = path.file_name();
                    names.push(file_name);
                }
                *(names_cpy.lock().unwrap()) = names;
            }
        });
        None
    }
    #[cfg(target_arch = "wasm32")]
    fn execute<F: std::future::Future<Output = ()> + 'static>(f: F) {
        wasm_bindgen_futures::spawn_local(f);
    }
}
