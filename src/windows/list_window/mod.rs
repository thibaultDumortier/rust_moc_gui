pub(crate) mod info_window;

use egui::{Context, ScrollArea, TextEdit, Ui};
use egui_extras::{Column, TableBuilder};
use moc::storage::u64idx::U64MocStore;
use std::collections::BTreeSet;

use crate::utils::commons::to_file;
use crate::utils::namestore::{self, get_store, list_ids, rename};

use self::info_window::InfoWindow;

use super::main_windows::unitary::lite_ui;
use super::Window;

#[derive(Default)]
pub struct InfoWindows {
    infouis: Vec<Box<InfoWindow>>,
    open: BTreeSet<String>,
    filenames: Vec<(usize, (String, usize))>,
    name: String,
}
impl InfoWindows {
    pub fn from_mocs(infouis: Vec<Box<InfoWindow>>) -> Self {
        let open = BTreeSet::new();
        let filenames = Vec::default();
        Self {
            infouis,
            open,
            filenames,
            name: String::from(""),
        }
    }

    pub fn checkboxes(&mut self, ui: &mut Ui) {
        let binding = get_store().read().unwrap().clone();
        for file in binding.iter() {
            let owned_file = (file.0.to_owned(), file.1.to_owned());
            if !self.filenames.contains(&owned_file) {
                self.filenames.push(owned_file);
            }
        }
        self.filenames.sort_by(|a, b| a.1 .1.cmp(&b.1 .1));

        let txt_h = 30.0;
        ui.vertical(|ui| {
            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::initial(300.0).at_least(100.0))
                .column(Column::initial(20.0).at_least(20.0))
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
                    body.rows(txt_h, self.filenames.len(), |row_index, mut row| {
                        row.col(|ui| {
                            let mut is_open = self
                                .open
                                .contains(&self.filenames.get(row_index).unwrap().1 .0);
                            ui.horizontal(|ui| {
                                ui.toggle_value(
                                    &mut is_open,
                                    &self.filenames.get(row_index).unwrap().1 .0,
                                )
                                .context_menu(|ui| {
                                    ui.menu_button("Unitary ops", |ui| lite_ui(ui, row_index));
                                    self.download(ui, row_index, "Download");
                                    ui.horizontal(|ui| {
                                        ui.add(TextEdit::singleline(&mut self.name).hint_text("Name"));
                                        if ui.button("rename").clicked() {
                                            let _ = rename(row_index, &self.name, self.filenames.get(row_index).unwrap().1.1);
                                        }
                                    });
                                });
                            });
                            set_open(
                                &mut self.open,
                                Box::leak(
                                    self.filenames
                                        .get(row_index)
                                        .unwrap()
                                        .1
                                         .0
                                        .to_string()
                                        .into_boxed_str(),
                                ),
                                is_open,
                            );
                        });
                        row.col(|ui| {
                            self.download(ui, row_index, "📥");
                        });
                        row.col(|ui| {
                            if ui.button("❌").clicked() {
                                let id = self.filenames.get(row_index).unwrap().0;
                                let _ = namestore::drop(id);
                                let _ = U64MocStore.drop(id);
                            }
                        });
                    })
                })
        });
    }

    pub fn windows(&mut self, ctx: &Context) {
        let Self {
            infouis,
            open,
            filenames: _,
            name: _,
        } = self;
        for infoui in infouis {
            let mut is_open = open.contains(infoui.name());
            infoui.show(ctx, &mut is_open);
            set_open(open, infoui.name(), is_open);
        }
    }

    ///////////////
    // UTILITIES //

    fn download(&mut self, ui: &mut Ui, id: usize, title: &str) {
        ui.menu_button(title, |ui| {
            if ui.button("FITS").clicked() {
                let _ = to_file(
                    &self.filenames.get(id).unwrap().1 .0,
                    ".fits",
                    "application/fits",
                    U64MocStore
                        .to_fits_buff(self.filenames.get(id).unwrap().0, None)
                        .unwrap(),
                );
            }
            if ui.button("ASCII").clicked() {
                let _ = to_file(
                    &self.filenames.get(id).unwrap().1 .0,
                    ".txt",
                    "text/plain",
                    U64MocStore
                        .to_ascii_str(self.filenames.get(id).unwrap().0, None)
                        .unwrap()
                        .into_bytes()
                        .into_boxed_slice(),
                );
            }
            if ui.button("JSON").clicked() {
                let _ = to_file(
                    &self.filenames.get(id).unwrap().1 .0,
                    ".json",
                    "application/json",
                    U64MocStore
                        .to_json_str(self.filenames.get(id).unwrap().0, None)
                        .unwrap()
                        .into_bytes()
                        .into_boxed_slice(),
                );
            }
        });
    }
}

// -----------------------------------------------------------

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

// -----------------------------------------------------------

#[derive(Default)]
pub struct ListUi {
    infouis: InfoWindows,
}

impl ListUi {
    /// Show the app ui (menu bar and windows).
    pub fn ui(&mut self, ctx: &Context) {
        self.updater(ctx);
        self.desktop_ui(ctx);
    }

    fn desktop_ui(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::trace!(ui);
            self.infoui_list_ui(ui);
        });

        self.show_windows(ctx);
    }

    /// Show the open windows.
    fn show_windows(&mut self, ctx: &Context) {
        self.infouis.windows(ctx);
    }

    fn infoui_list_ui(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                self.infouis.checkboxes(ui);
            });
        });
    }

    fn updater(&mut self, ctx: &Context) {
        if list_ids().unwrap().len() != self.infouis.infouis.len() {
            let mut mocs: Vec<Box<InfoWindow>> = Vec::default();
            for id in list_ids().unwrap() {
                mocs.push(Box::new(InfoWindow::new(ctx, id).unwrap()));
            }
            self.infouis = InfoWindows::from_mocs(mocs)
        }
    }
}
