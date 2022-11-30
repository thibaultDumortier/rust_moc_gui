#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::{commons::*, window_options};
use crate::loaders::{store, store::get_store};
use crate::uis::{creationui::*, opui::*};

use eframe::egui;
use egui::menu;
use egui::Ui;
use egui_extras::{Size, TableBuilder};
use wasm_bindgen::prelude::wasm_bindgen;

//Import javascript log function
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

//An operation enumerator
enum UiMenu {
    One,
    Two,
    List,
    Crea,
    Test,
}
impl Default for UiMenu {
    fn default() -> Self {
        UiMenu::List
    }
}
impl PartialEq for UiMenu {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (UiMenu::One, UiMenu::One)
                | (UiMenu::Two, UiMenu::Two)
                | (UiMenu::List, UiMenu::List)
                | (UiMenu::Crea, UiMenu::Crea)
        )
    }
}

//FileApp struct
#[derive(Default)]
pub struct FileApp {
    operation: UiMenu,
    error: Option<String>,
    creation: CreationUis,
    opui: OpUis,
}
impl eframe::App for FileApp {
    /*
        update: function of FileApp struct from eframe::App
        Description: A function updating the state of the application
        Parameters:
            ctx: &equi::Context, the app's context
            frame is unused but mandatory
        Returns: ()
    */
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            egui::trace!(ui);
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.operation, UiMenu::List, "MOC list");
                ui.selectable_value(&mut self.operation, UiMenu::Crea, "MOC creation");
                ui.selectable_value(&mut self.operation, UiMenu::One, "1 MOC operation");
                ui.selectable_value(&mut self.operation, UiMenu::Two, "2 MOCs operation");
                ui.selectable_value(&mut self.operation, UiMenu::Test, "Test");
            });
            ui.end_row();

            ui.separator();
            match &self.operation {
                UiMenu::One => self.opui.moc_op1(ui),
                UiMenu::Two => self.opui.moc_op2(ui),
                UiMenu::List => self.list_ui(ui),
                UiMenu::Crea => self.creation.creation_ui(ui),
                UiMenu::Test => window_options::WindowOptions::default().show(ctx),
            }
        });
    }
}
impl FileApp {
    /*
        new: function of FileApp struct
        Description: A function handling the contents of the top bar
        Parameters: None
        Returns: FileApp
    */
    pub fn new() -> Self {
        FileApp {
            operation: UiMenu::default(),
            error: None,
            creation: CreationUis::default(),
            opui: OpUis::default(),
        }
    }

    /*
        bar_contents: function of FileApp struct
        Description: A function handling the contents of the top bar
        Parameters:
            ui: Ui, the ui from the app
        Returns: ()
    */
    fn bar_contents(&mut self, ui: &mut Ui) {
        egui::widgets::global_dark_light_mode_switch(ui);

        ui.separator();

        menu::bar(ui, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Files", |ui| {
                    ui.menu_button("Load", |ui| {
                        if ui.button("FITS").clicked() {
                            match load(&["fits"], Qty::Space) {
                                Ok(_) => self.error = None,
                                Err(e) => {
                                    self.error = Some(e);
                                }
                            }
                        }
                        ui.menu_button("JSON", |ui| {
                            if ui.button("Space").clicked() {
                                assert!(load(&["json"], Qty::Space).is_ok());
                            }
                            if ui.button("Time").clicked() {
                                assert!(load(&["json"], Qty::Time).is_ok());
                            }
                            if ui.button("Spacetime").clicked() {
                                assert!(load(&["json"], Qty::Timespace).is_ok());
                            }
                        });
                        ui.menu_button("ASCII", |ui| {
                            if ui.button("Space").clicked() {
                                assert!(load(&["ascii", "txt"], Qty::Space).is_ok());
                            }
                            if ui.button("Time").clicked() {
                                assert!(load(&["ascii", "txt"], Qty::Time).is_ok());
                            }
                            if ui.button("Spacetime").clicked() {
                                assert!(load(&["ascii", "txt"], Qty::Timespace).is_ok());
                            }
                        });
                    })
                });
            });
            if self.error.is_some() {
                ui.separator();
                ui.label(self.error.as_ref().unwrap());
            }
        });
    }

    fn list_ui(&mut self, ui: &mut Ui) {
        let mut filenames: Vec<String> = Vec::default();
        for file in get_store().read().unwrap().iter() {
            filenames.push(file.0.to_string());
        }
        let txt_h = 30.0;
        ui.vertical(|ui| {
            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Size::initial(300.0).at_least(100.0))
                .column(Size::initial(20.0).at_least(20.0))
                .column(Size::remainder().at_least(20.0))
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Name");
                    });
                    header.col(|ui| {
                        ui.heading("📥");
                    });
                    header.col(|ui| {
                        ui.heading("❌");
                    });
                })
                .body(|body| {
                    body.rows(txt_h, filenames.len(), |row_index, mut row| {
                        row.col(|ui| {
                            ui.label(filenames.get(row_index).unwrap());
                        });
                        row.col(|ui| {
                            ui.menu_button("📥", |ui| {
                                if ui.button("FITS").clicked() {
                                    if let Err(e) = to_fits_file(filenames.get(row_index).unwrap())
                                    {
                                        self.error = Some(e);
                                    }
                                }
                                if ui.button("ASCII").clicked()
                                    && to_ascii_file(filenames.get(row_index).unwrap(), Some(0))
                                        .is_err()
                                {
                                    self.error =
                                        Some("Error when trying to download file".to_string());
                                }
                                if ui.button("JSON").clicked()
                                    && to_json_file(filenames.get(row_index).unwrap(), Some(0))
                                        .is_err()
                                {
                                    self.error =
                                        Some("Error when trying to download file".to_string());
                                }
                            });
                        });
                        row.col(|ui| {
                            if ui.button("❌").clicked()
                                && store::drop(filenames.get(row_index).unwrap()).is_err()
                            {
                                self.error = Some("Error when trying to remove file".to_string());
                            }
                        });
                    })
                })
        });
    }
}
