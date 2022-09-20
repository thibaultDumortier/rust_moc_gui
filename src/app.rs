#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use moc::deser::fits::{from_fits_ivoa, MocIdxType, MocQtyType, MocType};
use moc::elemset::range::MocRanges;
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
use wasm_bindgen::JsValue;

#[derive(Default, Clone)]
pub struct UploadedFiles {
    name: String,
    data: InternalMoc,
    chosen: bool,
}

#[derive(Default)]
pub struct FileApp {
    files: Arc<Mutex<Vec<UploadedFiles>>>,
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

            // #[cfg(not(target_arch = "wasm32"))]
            // if let Some(picked_path) = &self.picked_path {
            //     for str in picked_path {
            //         ui.horizontal(|ui| {
            //             ui.label("Picked file:");
            //             ui.monospace(str);
            //         });
            //     }
            // }
            //#[cfg(target_arch = "wasm32")]
            {
                let files = self.files.lock().unwrap().to_vec();
                for mut file in files {
                    ui.horizontal(|ui| {
                        ui.label("Picked file:");
                        ui.monospace(file.name.as_str());
                        if !file.chosen {
                            if ui.button("chose").clicked() {
                                file.chosen = true;
                            }
                        } else if file.chosen {
                            if ui.button("cancel").clicked() {
                                file.chosen = false;
                            }
                        }
                    });
                }
            }
        });
    }
}
impl FileApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        FileApp {
            files: Arc::new(Mutex::new(Default::default())),
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
                    file.data = match from_fits_ivoa(Cursor::new(file_content.as_ref()))
                        .map_err(|e| JsValue::from_str(&e.to_string()))
                        .unwrap()
                    {
                        MocIdxType::U16(moc) => from_fits(moc),
                        MocIdxType::U32(moc) => from_fits(moc),
                        MocIdxType::U64(moc) => from_fits(moc),
                    }
                    .map_err(|e| JsValue::from_str(&e.to_string()))
                    .unwrap();
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

type SMOC = RangeMOC<u64, Hpx<u64>>;

#[derive(Clone)]
enum InternalMoc {
    Space(SMOC),
}
impl Default for InternalMoc {
    fn default() -> Self {
        InternalMoc::Space(SMOC::new(0, MocRanges::default()))
    }
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
