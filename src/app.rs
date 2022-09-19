#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use moc::deser::fits::{from_fits_ivoa, MocIdxType, MocQtyType, MocType};
use moc::idx::Idx;
use moc::moc::range::op::convert::convert_to_u64;
use moc::moc::range::RangeMOC;
use moc::moc::{CellMOCIntoIterator, CellMOCIterator, RangeMOCIterator};
use moc::qty::Hpx;
use rfd::AsyncFileDialog;
use std::error::Error;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::console::log;

#[derive(Default)]
pub struct FileApp {
    names: Arc<Mutex<Vec<String>>>,
    data: Arc<Mutex<Vec<InternalMoc>>>,
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
            data: Arc::new(Mutex::new(Default::default())),
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

    // #[cfg(not(target_arch = "wasm32"))]
    // pub fn fileclick(&mut self) -> Option<Vec<PathBuf>> {
    //     if let Some(path) = rfd::FileDialog::new()
    //         .add_filter("MOCs", &["fits", "ascii", "json", "txt"])
    //         .pick_files()
    //     {
    //         return Some(path);
    //     } else {
    //         return None;
    //     }
    // }

    //#[cfg(target_arch = "wasm32")]
    pub fn fileclick(&mut self) -> Option<Vec<PathBuf>> {
        use eframe::epaint::text::cursor;

        let task = AsyncFileDialog::new()
            .add_filter("MOCs", &["fits", "ascii", "json", "txt"])
            .pick_files();
        let names_cpy = self.names.clone();
        let data_cpy = self.data.clone();

        Self::execute(async move {
            let file = task.await;
            let mut names: Vec<String> = Default::default();
            let mut data: Vec<InternalMoc> = Default::default();

            if let Some(file) = file {
                // If you care about wasm support you just read() the file
                for path in file {
                    //Reads name and adds it to be shown to user
                    let file_name = path.file_name();
                    names.push(file_name);
                    //Reads file contents and adds it to the data
                    let file_content = path.read().await;
                    data.push(
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
                }
                *(names_cpy.lock().unwrap()) = names;
                *(data_cpy.lock().unwrap()) = data;
            }
        });
        None
    }
    //#[cfg(target_arch = "wasm32")]
    fn execute<F: std::future::Future<Output = ()> + 'static>(f: F) {
        wasm_bindgen_futures::spawn_local(f);
    }
}

type SMOC = RangeMOC<u64, Hpx<u64>>;
enum InternalMoc {
    Space(SMOC),
}
fn from_fits<T: Idx>(moc: MocQtyType<T, Cursor<&[u8]>>) -> Result<InternalMoc, Box<dyn Error>> {
    match moc {
        MocQtyType::Hpx(moc) => from_fits_hpx(moc),
        MocQtyType::Time(_) => todo!(),
        MocQtyType::TimeHpx(_) => todo!(),
    }
}

fn from_fits_hpx<T: Idx>(
    moc: MocType<T, Hpx<T>, Cursor<&[u8]>>,
) -> Result<InternalMoc, Box<dyn Error>> {
    let moc: SMOC = match moc {
        MocType::Ranges(moc) => convert_to_u64::<T, Hpx<T>, _, Hpx<u64>>(moc).into_range_moc(),
        MocType::Cells(moc) => {
            convert_to_u64::<T, Hpx<T>, _, Hpx<u64>>(moc.into_cell_moc_iter().ranges())
                .into_range_moc()
        }
    };
    Ok(InternalMoc::Space(moc))
}
