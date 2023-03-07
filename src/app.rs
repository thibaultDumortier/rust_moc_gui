use std::collections::HashMap;

use crate::utils::commons::*;
use crate::views::infoui::ListUi;
use crate::views::windowed::{SubUiWindow, UiMenu};

use eframe::egui;
use egui::menu;
use egui::Ui;
use moc::storage::u64idx::common::MocQType;
use wasm_bindgen::prelude::wasm_bindgen;

//Import javascript log function
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

//FileApp struct
#[derive(Default)]
pub struct FileApp {
    list: ListUi,
    open_windows: HashMap<UiMenu, SubUiWindow>,
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
            let uis = [UiMenu::Crea, UiMenu::One, UiMenu::Two];
            for ui in uis {
                if self.open_windows.is_empty() {
                    self.open_windows = HashMap::new();
                    break;
                }
                let mut is_open = self.open_windows.contains_key(&ui);
                if is_open {
                    self.open_windows
                        .get(&ui)
                        .unwrap()
                        .to_owned()
                        .show(ctx, &mut is_open);
                }
                self.set_open(&ui.clone(), is_open);
            }

            ui.horizontal(|ui| {
                if ui.button("MOC creation").clicked() {
                    if !self.open_windows.contains_key(&UiMenu::Crea) {
                        self.open_windows.insert(
                            UiMenu::Crea,
                            SubUiWindow::new(UiMenu::Crea).unwrap(), //NO ERRORS SHOULD HAPPEN HERE
                        );
                    } else if self.open_windows.contains_key(&UiMenu::Crea) {
                        self.open_windows.remove(&UiMenu::Crea);
                    }
                }
                if ui.button("1 MOC operation").clicked() {
                    if !self.open_windows.contains_key(&UiMenu::One) {
                        self.open_windows.insert(
                            UiMenu::One,
                            SubUiWindow::new(UiMenu::One).unwrap(), //NO ERRORS SHOULD HAPPEN HERE
                        );
                    } else if self.open_windows.contains_key(&UiMenu::One) {
                        self.open_windows.remove(&UiMenu::One);
                    }
                }   
                if ui.button("2 MOCs operation").clicked() {
                    if !self.open_windows.contains_key(&UiMenu::Two) {
                        self.open_windows.insert(
                            UiMenu::Two,
                            SubUiWindow::new(UiMenu::Two).unwrap(), //NO ERRORS SHOULD HAPPEN HERE
                        );
                    } else if self.open_windows.contains_key(&UiMenu::Two) {
                        self.open_windows.remove(&UiMenu::Two);
                    }
                }
            });
            ui.end_row();

            ui.separator();
            let _ = self.list.list_ui(ctx, ui).map_err(|e| self.err(&e));
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
                            assert!(load(&["fits"], MocQType::Space).is_ok());
                        }
                        ui.menu_button("JSON", |ui| {
                            if ui.button("Space").clicked() {
                                assert!(load(&["json"], MocQType::Space).is_ok());
                            }
                            if ui.button("Time").clicked() {
                                assert!(load(&["json"], MocQType::Time).is_ok());
                            }
                            if ui.button("Spacetime").clicked() {
                                assert!(load(&["json"], MocQType::TimeSpace).is_ok());
                            }
                        });
                        ui.menu_button("ASCII", |ui| {
                            if ui.button("Space").clicked() {
                                assert!(load(&["ascii", "txt"], MocQType::Space).is_ok());
                            }
                            if ui.button("Time").clicked() {
                                assert!(load(&["ascii", "txt"], MocQType::Time).is_ok());
                            }
                            if ui.button("Spacetime").clicked() {
                                assert!(load(&["ascii", "txt"], MocQType::TimeSpace).is_ok());
                            }
                        });
                    })
                });
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn err(&mut self, msg: &str) {
        use rfd::MessageDialog;

        let m = MessageDialog::new()
            .set_buttons(rfd::MessageButtons::Ok)
            .set_title("Error !")
            .set_description(msg);
        m.show();
    }

    #[cfg(target_arch = "wasm32")]
    fn err(&mut self, msg: &str) {
        use rfd::AsyncMessageDialog;

        let m = AsyncMessageDialog::new()
            .set_buttons(rfd::MessageButtons::Ok)
            .set_title("Error !")
            .set_description(msg);
        m.show();
    }

    fn set_open(&mut self, key: &UiMenu, is_open: bool) {
        if !is_open {
            self.open_windows.remove(key);
        }
    }
}
