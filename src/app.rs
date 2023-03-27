use crate::utils::commons::*;
use crate::windows::list_window::ListUi;
use crate::windows::main_windows::ToolsUi;

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

#[derive(Default)]
pub struct SubUiApp {
    subui_windows: ToolsUi,
}

impl eframe::App for SubUiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.subui_windows.ui(ctx);
    }
}

// -------------------------------------------------------------------

#[derive(Default)]
pub struct State {
    subui: SubUiApp,

    selected_anchor: String,
}

//FileApp struct
#[derive(Default)]
pub struct FileApp {
    list: ListUi,
    state: State,
}
impl eframe::App for FileApp {
    //////////////////////
    // Eframe functions //

    // #Definition
    //      A function updating the state of the application at a given interval
    // #Args:
    //  *    ctx: &equi::Context, the app's context
    //  *    frame is unused but mandatory
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F11)) {
            frame.set_fullscreen(!frame.info().window_info.fullscreen);
        }

        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            egui::trace!(ui);
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ui);
            });
        });

        let _ = self.list.ui(ctx)/*.map_err(|e| self.err(&e))*/;

        self.show_selected_app(ctx, frame);

        // On web, the browser controls `pixels_per_point`.
        if !frame.is_web() {
            egui::gui_zoom::zoom_with_keyboard_shortcuts(ctx, frame.info().native_pixels_per_point);
        }
    }
}
impl FileApp {
    /////////////////////
    // Basic functions //

    fn apps_iter_mut(&mut self) -> impl Iterator<Item = (&str, &str, &mut dyn eframe::App)> {
        let vec = vec![("", "subui", &mut self.state.subui as &mut dyn eframe::App)];

        vec.into_iter()
    }

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

    fn show_selected_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut found_anchor = false;
        let selected_anchor = self.state.selected_anchor.clone();
        for (_name, anchor, app) in self.apps_iter_mut() {
            if anchor == selected_anchor || ctx.memory(|mem| mem.everything_is_visible()) {
                app.update(ctx, frame);
                found_anchor = true;
            }
        }
        if !found_anchor {
            self.state.selected_anchor = "subui".into();
        }
    }
}
