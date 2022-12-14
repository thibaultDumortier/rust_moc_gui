#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::commons::*;
use crate::views::infoui::ListUi;
use crate::views::{creationui::*, opui::*};

use eframe::egui;
use egui::menu;
use egui::Ui;
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
    list: ListUi,
}
impl eframe::App for FileApp {
    //////////////////////
    // Eframe functions //

    // #Definition
    //      A function updating the state of the application at a given interval
    // #Args:
    //  *    ctx: &equi::Context, the app's context
    //  *    frame is unused but mandatory
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
            });
            ui.end_row();

            ui.separator();
            match &self.operation {
                UiMenu::One => self.error = self.opui.moc_op1(ui, &self.error),
                UiMenu::Two => self.error = self.opui.moc_op2(ui, &self.error),
                UiMenu::List => self.error = self.list.list_ui(ctx, ui, &self.error),
                UiMenu::Crea => self.creation.creation_ui(ui),
            }
        });
    }
}
impl FileApp {
    /////////////////////
    // Basic functions //

    // #Definition
    //      A function handling the contents of the top bar
    // #Args
    //  *   ui: Ui, the ui from the app
    fn bar_contents(&mut self, ui: &mut Ui) {
        egui::widgets::global_dark_light_mode_switch(ui);

        ui.separator();

        menu::bar(ui, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Files", |ui| {
                    ui.menu_button("Load", |ui| {
                        if ui.button("FITS").clicked() {
                            //Qty::Space here is a default it is not actually used
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
}
