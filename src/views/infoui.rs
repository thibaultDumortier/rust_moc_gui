use std::collections::HashMap;

use egui::Ui;
use egui_extras::{Column, TableBuilder};

use crate::{
    commons::{to_ascii_file, to_fits_file, to_json_file, Qty},
    models::store::{self, get_store},
};

#[derive(Clone, PartialEq, Default, PartialOrd, Ord, Eq)]
pub struct InfoWindow {
    pub title: String,
}

impl InfoWindow {
    pub fn new(title: String) -> Self {
        Self { title }
    }

    pub fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        let mut window = egui::Window::new(self.title.clone())
            .id(egui::Id::new(self.title.clone())) // required since we change the title
            .resizable(true)
            .title_bar(true)
            .enabled(true);
        window = window.open(open);
        window.show(ctx, |ui| self.ui(ui));
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        let qty = store::get_qty(&self.title).unwrap();

        ui.horizontal(|ui| {
            ui.label("MOC type:");
            ui.label(qty.to_string().as_str());
        });

        match qty {
            Qty::Space => ui.label("Possible operations include:\n-All solo operations.\n-All same type duo operations.\n-SFold with a SpaceTime MOC."),
            Qty::Time => ui.label("Possible operations include:\n-Complement and degrade.\n-All same type duo operations\n-TFold with a SpaceTime MOC."),
            Qty::Timespace => ui.label("Possible operations include:\n-No solo operations.\n-All same type duo operations.\n-SFold or TFold depending on the other MOC's type."),
        };
    }
}

#[derive(Default)]
pub struct ListUi {
    open_windows: HashMap<String, InfoWindow>,
}
impl ListUi {
    pub(crate) fn list_ui(
        &mut self,
        ctx: &egui::Context,
        ui: &mut Ui,
        e: &Option<String>,
    ) -> Option<String> {
        let mut err = e.to_owned();

        for moc in store::list_mocs().unwrap() {
            if self.open_windows.is_empty() {
                self.open_windows = HashMap::new();
                break;
            }
            let mut is_open = self.open_windows.contains_key(&moc);
            if is_open {
                self.open_windows
                    .get(&moc)
                    .unwrap()
                    .to_owned()
                    .show(ctx, &mut is_open);
            }
            self.set_open(&moc.clone(), is_open);
        }

        let mut filenames: Vec<String> = Vec::default();
        for file in get_store().read().unwrap().iter() {
            filenames.push(file.0.to_string());
        }
        let txt_h = 30.0;
        ui.vertical(|ui| {
            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::initial(300.0).at_least(100.0))
                .column(Column::initial(20.0).at_least(20.0))
                .column(Column::remainder().at_least(20.0))
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
                            if ui.button(filenames.get(row_index).unwrap()).clicked() {
                                let name = filenames.get(row_index).unwrap().to_string();
                                // If an information window doesn't exist, create one.
                                if !self.open_windows.contains_key(&name) {
                                    self.open_windows
                                        .insert(name.clone(), InfoWindow::new(name));
                                } else if self.open_windows.contains_key(&name) {
                                    self.open_windows.remove(&name);
                                }
                            };
                        });
                        row.col(|ui| {
                            ui.menu_button("📥", |ui| {
                                if ui.button("FITS").clicked() {
                                    let _ = to_fits_file(filenames.get(row_index).unwrap())
                                        .map_err(|e| err = Some(e));
                                }
                                if ui.button("ASCII").clicked() {
                                    let _ =
                                        to_ascii_file(filenames.get(row_index).unwrap(), Some(0))
                                            .map_err(|e| err = Some(e));
                                }
                                if ui.button("JSON").clicked() {
                                    let _ =
                                        to_json_file(filenames.get(row_index).unwrap(), Some(0))
                                            .map_err(|e| err = Some(e));
                                }
                            });
                        });
                        row.col(|ui| {
                            if ui.button("❌").clicked() {
                                let _ = store::drop(filenames.get(row_index).unwrap())
                                    .map_err(|e| err = Some(e));
                            }
                        });
                    })
                })
        });
        err
    }
    fn set_open(&mut self, key: &str, is_open: bool) {
        if !is_open {
            self.open_windows.remove(key);
        }
    }
}
