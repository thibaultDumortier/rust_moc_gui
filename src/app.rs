#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::commons::*;
use crate::op2::*;

use eframe::egui;
use egui::Ui;
use moc::deser::fits::{from_fits_ivoa, MocIdxType};
use rfd::AsyncFileDialog;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Default, Clone)]
pub struct UploadedFiles {
    name: String,
    data: Option<InternalMoc>,
}
impl PartialEq for UploadedFiles {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Default)]
pub struct FileApp {
    files: Arc<Mutex<Vec<UploadedFiles>>>,
    picked_path: Option<Vec<String>>,
    picked_file: Option<UploadedFiles>,
    picked_second_file: Option<UploadedFiles>,
    operation: Op2,
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
            // An Op2eration combo box including Intersection and Union
            let sel_text = format!("{}", self.operation);
            egui::ComboBox::from_id_source("Op2eration_cbox")
                .selected_text(sel_text)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.operation, Op2::AND, "Intersection");
                    ui.selectable_value(&mut self.operation, Op2::OR, "Union");
                });

            //A file choosing combobox
            match self.operation {
                Op2::AND => self.moc_op2(ui, Op2::AND),
                Op2::OR => todo!(),
            }
        });
    }
}
impl FileApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        FileApp {
            files: Arc::new(Mutex::new(Default::default())),
            picked_path: None,
            picked_file: None,
            picked_second_file: None,
            operation: Op2::default(),
        }
    }
    fn bar_contents(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
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

    pub fn moc_op1(&mut self, ui: &mut Ui, op2: Op2) {
        if self.files.lock().unwrap().to_vec().is_empty() {
            ui.label("Pick a file!");
        } else {
            let files = self.files.lock().unwrap().to_vec();

            let mut sel_text = "pick one".to_string();
            let mut sel_text_2 = "pick one".to_string();
            if self.picked_file.is_some() {
                sel_text = format!("{}", self.picked_file.as_ref().unwrap().name);
            }
            if self.picked_second_file.is_some() {
                sel_text_2 = format!("{}", self.picked_second_file.as_ref().unwrap().name);
            }
            egui::ComboBox::from_id_source("file_cbox")
                .selected_text(sel_text.as_str())
                .show_ui(ui, |ui| {
                    for file in &files {
                        ui.selectable_value(
                            &mut self.picked_file,
                            Some(file.clone()),
                            file.clone().name,
                        );
                    }
                });
            if ui.button("Do Operation").clicked() {
                let l = self.picked_file.clone().unwrap().data.unwrap();
                let r = self.picked_second_file.clone().unwrap().data.unwrap();
                let res;
                res = match (l, r) {
                    (InternalMoc::Space(l), InternalMoc::Space(r)) => {
                        op2.perform_op2_on_smoc(&l, &r).map(InternalMoc::Space)
                    }
                };
                log(&format!("{:?}", res.unwrap().to_fits().to_vec()));
            };
        }
    }

    pub fn moc_op2(&mut self, ui: &mut Ui, op2: Op2) {
        if self.files.lock().unwrap().to_vec().is_empty() {
            ui.label("Pick a file!");
        } else {
            let files = self.files.lock().unwrap().to_vec();

            let mut sel_text = "pick one".to_string();
            let mut sel_text_2 = "pick one".to_string();
            if self.picked_file.is_some() {
                sel_text = format!("{}", self.picked_file.as_ref().unwrap().name);
            }
            if self.picked_second_file.is_some() {
                sel_text_2 = format!("{}", self.picked_second_file.as_ref().unwrap().name);
            }
            egui::ComboBox::from_id_source("file_cbox")
                .selected_text(sel_text.as_str())
                .show_ui(ui, |ui| {
                    for file in &files {
                        ui.selectable_value(
                            &mut self.picked_file,
                            Some(file.clone()),
                            file.clone().name,
                        );
                    }
                });
            egui::ComboBox::from_id_source("file_cbox_2")
                .selected_text(sel_text_2.as_str())
                .show_ui(ui, |ui| {
                    for file in &files {
                        ui.selectable_value(
                            &mut self.picked_second_file,
                            Some(file.clone()),
                            file.clone().name,
                        );
                    }
                });
            if ui.button("Do Operation").clicked() {
                let l = self.picked_file.clone().unwrap().data.unwrap();
                let r = self.picked_second_file.clone().unwrap().data.unwrap();
                let res;
                res = match (l, r) {
                    (InternalMoc::Space(l), InternalMoc::Space(r)) => {
                        op2.perform_op2_on_smoc(&l, &r).map(InternalMoc::Space)
                    }
                };
                log(&format!("{:?}", res.unwrap().to_fits().to_vec()));
            };
        }
    }

    //#[cfg(target_arch = "wasm32")]
    pub fn fileclick(&mut self) -> Option<Vec<PathBuf>> {
        let task = AsyncFileDialog::new()
            .add_filter("MOCs", &["fits", "ascii", "json", "txt"])
            .pick_files();
        let files_cpy = self.files.clone();

        Self::execute(async move {
            let handle = task.await;
            let mut files: Vec<UploadedFiles> = Default::default();

            if let Some(handle) = handle {
                // If you care about wasm support you just read() the file
                for path in handle {
                    let mut file = UploadedFiles::default();
                    //Reads name and adds it to be shown to user
                    let file_name = path.file_name();
                    file.name = file_name;
                    //Reads file contents and adds it to the data
                    let file_content = path.read().await;
                    file.data = Some(
                        match from_fits_ivoa(Cursor::new(file_content.as_ref()))
                            .map_err(|e| JsValue::from_str(&e.to_string()))
                            .unwrap()
                        {
                            MocIdxType::U16(moc) => from_fits(moc),
                            MocIdxType::U32(moc) => from_fits(moc),
                            MocIdxType::U64(moc) => from_fits(moc),
                        }
                        .map_err(|e| JsValue::from_str(&e.to_string()))
                        .unwrap(),
                    );
                    files.push(file);
                }
                *(files_cpy.lock().unwrap()) = files;
            }
        });
        None
    }
    //#[cfg(target_arch = "wasm32")]
    fn execute<F: std::future::Future<Output = ()> + 'static>(f: F) {
        wasm_bindgen_futures::spawn_local(f);
    }
}
