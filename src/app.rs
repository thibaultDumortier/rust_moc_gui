#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::commons::*;
use crate::loaders::{store, store::get_store};
use crate::op::creation::*;
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
enum Op {
    One,
    Two,
    List,
    Crea(CreationType),
}
impl Default for Op {
    fn default() -> Self {
        Op::One
    }
}
impl PartialEq for Op {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Op::One, Op::One) => true,
            (Op::Two, Op::Two) => true,
            (Op::List, Op::List) => true,
            _ => false,
        }
    }
}

//FileApp struct
#[derive(Default)]
pub struct FileApp {
    operation: Op,
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
                ui.selectable_value(&mut self.operation, Op::List, "MOC list");
                ui.selectable_value(
                    &mut self.operation,
                    Op::Crea(CreationType::Cone),
                    "MOC creation",
                );
                ui.selectable_value(
                    &mut self.operation,
                    Op::One,
                    "1 MOC operation",
                );
                ui.selectable_value(
                    &mut self.operation,
                    Op::Two,
                    "2 MOCs operation",
                );
            });
            ui.end_row();

            ui.separator();
            match &self.operation {
                Op::One => self.opui.moc_op1(ui),
                Op::Two => self.opui.moc_op2(ui),
                Op::List => self.list_ui(ui),
                Op::Crea(c) => self.creation_ui(ui, *c),
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
            operation: Op::default(),
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

    fn creation_ui(&mut self, ui: &mut Ui, crea: CreationType) {
        let sel_text = format!("{}", crea);

        ui.horizontal(|ui| {
            ui.label("Creation type :");
            egui::ComboBox::from_id_source("Creation_cbox")
                .selected_text(sel_text)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.operation, Op::Crea(CreationType::Cone), "Cone");
                    ui.selectable_value(&mut self.operation, Op::Crea(CreationType::Ring), "Ring");
                    ui.selectable_value(
                        &mut self.operation,
                        Op::Crea(CreationType::EllipticalCone),
                        "Eliptical cone",
                    );
                    ui.selectable_value(&mut self.operation, Op::Crea(CreationType::Zone), "Zone");
                    ui.selectable_value(&mut self.operation, Op::Crea(CreationType::Box), "Box");
                    ui.selectable_value(&mut self.operation, Op::Crea(CreationType::Polygon), "Polygon");
                });
        });

        match crea {
            CreationType::Cone => self.error = self.creation.cone_ui(ui, &self.error),
            CreationType::Ring => self.error = self.creation.ring_ui(ui, &self.error),
            CreationType::EllipticalCone => {
                self.error = self.creation.eliptical_ui(ui, &self.error)
            }
            CreationType::Zone => self.error = self.creation.zone_ui(ui, &self.error),
            CreationType::Box => self.error = self.creation.box_ui(ui, &self.error),
            CreationType::Polygon => self.error = self.creation.polygon_ui(ui, &self.error),
            _ => todo!(),
        };
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
                                if ui.button("FITS").clicked()
                                    && to_fits_file(filenames.get(row_index).unwrap()).is_err()
                                {
                                    self.error =
                                        Some("Error when trying to download file".to_string());
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