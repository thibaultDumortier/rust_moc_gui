use std::{borrow::Borrow, collections::HashMap};

use crate::utils::namestore::{self, get_name, get_store, list_names};
use egui::{Color32, Ui};
use egui_extras::{Column, TableBuilder};
use moc::storage::u64idx::common::MocQType;
use moc::storage::u64idx::U64MocStore;

use crate::utils::commons::{fmt_qty, to_file};

#[derive(Clone, PartialEq, Default, Eq)]
pub struct InfoWindow {
    pub id: usize,
    texture: Option<egui::TextureHandle>,
    info: String,
}

impl InfoWindow {
    pub fn new(ctx: &egui::Context, id: usize) -> Result<Self, String> {
        let mut texture: Option<egui::TextureHandle> = None;
        if let Ok(i) = U64MocStore.to_image(id, 150) {
            texture =
                // Load the texture only once.
                Some(ctx.load_texture(
                    "moc_img",
                    egui::ColorImage::from_rgba_unmultiplied([300, 150], i.borrow()),
                    Default::default(),
                ));
        }

        let mut info = String::default();
        match U64MocStore.get_qty_type(id) {
            Ok(qty) => match qty {
                MocQType::Space => {
                    if let Ok(s) = U64MocStore.get_smoc_depth(id) {
                        info = format!(
                            "Depth: {}, Coverage: {}",
                            s.to_string(),
                            U64MocStore.get_coverage_percentage(id).unwrap().to_string()
                        )
                    }
                }
                MocQType::Time => {
                    if let Ok(t) = U64MocStore.get_tmoc_depth(id) {
                        info = format!("Depth: {}", t.to_string())
                    }
                }
                MocQType::Frequency => {
                    return Err(String::from("Frequency MOCs are not supported"))
                } //TODO ADD FREQUENCY ERROR
                MocQType::TimeSpace => {
                    if let Ok(st) = U64MocStore.get_stmoc_depths(id) {
                        info = format!(
                            "Depth S: {}\nDepth T: {}",
                            st.0.to_string(),
                            st.1.to_string()
                        )
                    }
                }
            },
            Err(e) => return Err(e),
        }

        Ok(Self { id, texture, info })
    }

    pub fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        if let Ok(n) = get_name(self.id) {
            let mut window = egui::Window::new(n.clone())
                .id(egui::Id::new(n)) // required since we change the title
                .resizable(false)
                .title_bar(true)
                .enabled(true);
            window = window.open(open);
            window.show(ctx, |ui| self.ui(ui));
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        let qty = U64MocStore.get_qty_type(self.id).unwrap();

        ui.horizontal(|ui| {
            ui.label("MOC type:");
            ui.label(fmt_qty(qty));
        });

        match qty {
            MocQType::Space => {
                ui.label("Possible operations include:\n-All solo operations.\n-All same type duo operations.\n-SFold with a SpaceTime MOC.");
                ui.label(&self.info);
                let texture = &self.texture.clone().unwrap();
                ui.add(egui::Image::new(texture, texture.size_vec2()).bg_fill(Color32::WHITE));
            }
            MocQType::Time => {
                ui.label("Possible operations include:\n-Complement and degrade.\n-All same type duo operations\n-TFold with a SpaceTime MOC.");
                ui.label(&self.info);
            }
            MocQType::TimeSpace => {
                ui.label("Possible operations include:\n-No solo operations.\n-All same type duo operations.\n-SFold or TFold depending on the other MOC's type.");
                ui.label(&self.info);
            }
            MocQType::Frequency => todo!(),
        };
    }
}

#[derive(Default)]
pub struct ListUi {
    open_windows: HashMap<String, InfoWindow>,
}
impl ListUi {
    pub(crate) fn list_ui(&mut self, ctx: &egui::Context, ui: &mut Ui) -> Result<(), String> {
        for moc in list_names().unwrap() {
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

        let mut filenames: Vec<(&usize, &String)> = Vec::default();
        let binding = get_store().read().unwrap();
        for file in binding.iter() {
            filenames.push(file);
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
                            if ui.button(filenames.get(row_index).unwrap().1).clicked() {
                                let name = filenames.get(row_index).unwrap().1;
                                // If an information window doesn't exist, create one.
                                if !self.open_windows.contains_key(name) {
                                    self.open_windows.insert(
                                        name.clone(),
                                        InfoWindow::new(ctx, *filenames.get(row_index).unwrap().0)
                                            .unwrap(), //NO ERRORS SHOULD HAPPEN HERE
                                    );
                                } else if self.open_windows.contains_key(name) {
                                    self.open_windows.remove(name);
                                }
                            };
                        });
                        row.col(|ui| {
                            ui.menu_button("📥", |ui| {
                                if ui.button("FITS").clicked() {
                                    let _ = to_file(
                                        filenames.get(row_index).unwrap().1,
                                        ".fits",
                                        "application/fits",
                                        U64MocStore
                                            .to_fits_buff(
                                                *filenames.get(row_index).unwrap().0,
                                                None,
                                            )
                                            .unwrap(),
                                    );
                                }
                                if ui.button("ASCII").clicked() {
                                    let _ = to_file(
                                        filenames.get(row_index).unwrap().1,
                                        ".txt",
                                        "text/plain",
                                        U64MocStore
                                            .to_ascii_str(
                                                *filenames.get(row_index).unwrap().0,
                                                None,
                                            )
                                            .unwrap()
                                            .into_bytes()
                                            .into_boxed_slice(),
                                    );
                                }
                                if ui.button("JSON").clicked() {
                                    let _ = to_file(
                                        filenames.get(row_index).unwrap().1,
                                        ".json",
                                        "application/json",
                                        U64MocStore
                                            .to_json_str(*filenames.get(row_index).unwrap().0, None)
                                            .unwrap()
                                            .into_bytes()
                                            .into_boxed_slice(),
                                    );
                                }
                            });
                        });
                        row.col(|ui| {
                            if ui.button("❌").clicked() {
                                if let Ok(_) =
                                    U64MocStore.drop(*filenames.get(row_index).unwrap().0)
                                {
                                    let _ = namestore::drop(row_index); //NO ERROR ARE SUPPOSED TO HAPPEN HERE
                                }
                            }
                        });
                    })
                })
        });
        Ok(())
    }
    fn set_open(&mut self, key: &str, is_open: bool) {
        if !is_open {
            self.open_windows.remove(key);
        }
    }
}
