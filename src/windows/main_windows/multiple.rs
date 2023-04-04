use crate::controllers::op2::*;
use crate::utils::commons::{err, fmt_qty};
use crate::utils::namestore::{get_last, get_name, get_store, list_names};

use eframe::egui;
use egui::{TextEdit, Ui};
use moc::storage::u64idx::common::MocQType;
use moc::storage::u64idx::U64MocStore;

use crate::windows::{View, Window};

#[derive(Default, Clone, Eq, PartialEq)]
pub struct MultipleUi {
    name: String,
    operation: Op2,
    deg: u8,
    picked_file: Option<usize>,
    picked_second_file: Option<usize>,
}

impl Window for MultipleUi {
    fn name(&self) -> &'static str {
        "MOC multiple logical operations"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(false)
            .show(ctx, |ui| {
                use crate::windows::View as _;
                self.ui(ui);
            });
    }
}

impl MultipleUi {
    // #Definition
    //      A function handling operations on a stored MOC.
    // #Args
    //  *   `ui`: The ui from the app.
    fn op_two_ui(&mut self, ui: &mut Ui) {
        // An operation combo box including Intersection and Union
        if self.picked_file.is_some() && self.picked_second_file.is_some() {
            let l = self.picked_file.unwrap();
            let r = self.picked_second_file.unwrap();

            if files_have_same_type(l, r) {
                if self.operation.eq(&Op2::SFold) || self.operation.eq(&Op2::TFold) {
                    self.operation = Op2::Intersection;
                }
                let sel_text = format!("{}", self.operation);

                ui.label("Operation :");
                egui::ComboBox::from_id_source("Operation_cbox")
                    .selected_text(sel_text)
                    .show_ui(ui, |ui| {
                        if files_have_same_type(l, r) {
                            ui.selectable_value(
                                &mut self.operation,
                                Op2::Intersection,
                                "Intersection",
                            );
                            ui.selectable_value(&mut self.operation, Op2::Minus, "Minus");
                            ui.selectable_value(&mut self.operation, Op2::Union, "Union");
                            ui.selectable_value(&mut self.operation, Op2::Difference, "Difference");
                        }
                    });
            } else if files_have_stmoc(l, r) {
                ui.horizontal(|ui| {
                    if have_space(l, r) {
                        ui.label("Operation:");
                        ui.add_enabled(false, egui::widgets::Button::new("SFold"));
                        ui.end_row();
                        self.operation = Op2::SFold;
                    } else if have_time(l, r) {
                        ui.label("Operation:");
                        ui.add_enabled(false, egui::widgets::Button::new("TFold"));
                        ui.end_row();
                        self.operation = Op2::TFold;
                    }
                });
            } else {
                ui.label("Files need to be of same type or Space/Time and SpaceTime (for folds)");
            }
        } else {
            ui.label("Pick files on which to do operation");
        }
    }

    // #Definition
    //      A function that creates comboboxes.
    // #Args
    //  *   ui: Ui, the ui from the app.
    //  *   sel_text: &str, the text to show in the combo box.
    //  *   id: &str, the combobox gui ID.
    //  *   op: Option<u8>, to know if there needs to be multiple selected mocs.
    fn make_cbox(&mut self, ui: &mut Ui, sel_text: &str, id: &str, op: Option<u8>) {
        egui::ComboBox::from_id_source(id)
            .selected_text(sel_text)
            .show_ui(ui, |ui| {
                for file in get_store().read().unwrap().iter() {
                    if op.is_none() {
                        ui.selectable_value(&mut self.picked_file, Some(*file.0), &file.1 .0);
                    } else {
                        ui.selectable_value(
                            &mut self.picked_second_file,
                            Some(*file.0),
                            &file.1 .0,
                        );
                    }
                }
            });
    }
}

impl View for MultipleUi {
    // #Definition
    //      A function creating the UI for operations on 2 stored MOCs.
    // #Args
    //  *   `ui`: The ui from the app.
    //  *   `e`: an optional String in case of past errors to keep it visible until change
    fn ui(&mut self, ui: &mut Ui) {
        let op: Op2 = self.operation;

        // If no file has been imported yet.
        if list_names().unwrap().len() < 2 {
            ui.label("Pick at least 2 files!");
        // If files have been imported and can be chosen from.
        } else {
            let sel_text: String;
            let sel_text_2: String;
            if self.picked_file.is_some() {
                if let Ok(txt) = get_name(self.picked_file.unwrap()) {
                    sel_text = txt
                } else {
                    self.picked_file = Some(get_last(0).unwrap().0);
                    sel_text = get_name(self.picked_file.unwrap())
                        .map_err(|e| err(&e))
                        .unwrap();
                }
            } else {
                self.picked_file = Some(get_last(0).unwrap().0);
                sel_text = get_name(self.picked_file.unwrap())
                    .map_err(|e| err(&e))
                    .unwrap();
            }
            if self.picked_second_file.is_some() {
                if let Ok(txt) = get_name(self.picked_second_file.unwrap()).map_err(|e| return e) {
                    sel_text_2 = txt
                } else {
                    self.picked_second_file = Some(get_last(1).unwrap().0);
                    sel_text_2 = get_name(self.picked_second_file.unwrap())
                        .map_err(|e| err(&e))
                        .unwrap();
                }
            } else {
                self.picked_second_file = Some(get_last(1).unwrap().0);
                sel_text_2 = get_name(self.picked_second_file.unwrap())
                    .map_err(|e| err(&e))
                    .unwrap();
            }

            // The small paragraph before the match sets a grid layout to have every element aligned.
            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([5.0, 4.0])
                .striped(false)
                .show(ui, |ui| {
                    //Combo boxes containing the different files that can be picked from
                    ui.label("First MOC :");
                    self.make_cbox(ui, &sel_text, "file_cbox", None);
                    ui.end_row();
                    ui.label("Second MOC :");
                    self.make_cbox(ui, &sel_text_2, "file_cbox_2", Some(1));
                    ui.end_row();

                    self.op_two_ui(ui);
                    ui.end_row();

                    if self.picked_file.is_some() && self.picked_second_file.is_some() {
                        let l = self.picked_file.unwrap();
                        let r = self.picked_second_file.unwrap();
                        if files_have_same_type(l, r) || files_have_stmoc(l, r) {
                            ui.label("New MOC name :");
                            ui.add(TextEdit::singleline(&mut self.name).hint_text("Name"));
                            ui.end_row();

                            //Button launching the operation
                            if ui.button("Launch").clicked() {
                                let mut l = self.picked_file.as_ref().unwrap();
                                let mut r = self.picked_second_file.as_ref().unwrap();
                                if matches!(
                                    U64MocStore.get_qty_type(self.picked_file.unwrap()),
                                    Ok(MocQType::TimeSpace)
                                ) {
                                    std::mem::swap(&mut r, &mut l);
                                }
                                if self.name.is_empty() {
                                    self.name = format!("{}_{}_{}", op, l, r);
                                }
                                let _ = op2(*l, *r, op, &self.name).map_err(|e| err(&e));
                                self.name = String::default();
                            };
                        }
                    }
                });
        }
    }
}

// #Definition
//      a view showing buttons of every possible 2 MOCs operation when right clicking
// #Args
//  *   `l`: the index of the first MOC
//  *   `r`: the index of the second MOC
//  *   `ui`: the ui used by the app
pub(crate) fn lite_mult_ui(ui: &mut Ui, l: usize, r: usize) {
    if files_have_same_type(l, r) {
        if ui.button("Intersection").clicked() {
            lite_op(l, r, Op2::Intersection);
        };
        if ui.button("Minus").clicked() {
            lite_op(l, r, Op2::Minus);
        };
        if ui.button("Union").clicked() {
            lite_op(l, r, Op2::Union);
        };
        if ui.button("Difference").clicked() {
            lite_op(l, r, Op2::Difference);
        };
    } else if files_have_stmoc(l, r) {
        if have_space(l, r) {
            if ui.button("SFold").clicked() {
                lite_op(l, r, Op2::SFold);
            };
        } else if have_time(l, r) {
            if ui.button("TFold").clicked() {
                lite_op(l, r, Op2::TFold);
            };
        }
    } else {
        ui.label("Mocs need to be of same type or with 1 STMOC");
    }
}

// #Definition
//      the button launching the operation
// #Args
//  *   `l`: the index of the first MOC
//  *   `r`: the index of the second MOC
//  *   `operation`: the operation that needs to be applied on the MOCs
// #Errors
//      may show an error message coming from the op2 function
fn lite_op(mut l: usize, mut r: usize, operation: Op2) {
    //Button launching the operation
    if matches!(U64MocStore.get_qty_type(l), Ok(MocQType::TimeSpace)) {
        std::mem::swap(&mut r, &mut l);
    }
    let name = format!(
        "{}_{}_{}",
        operation,
        get_name(l).unwrap(),
        get_name(r).unwrap()
    );
    let _ = op2(l, r, operation, &name).map_err(|e| err(&e));
}

// #Definitions
//  *   files_have_stmoc: a simple check to see if a space time MOC is present in the 2 selected MOCs.
//  *   files_have_same_type: a simple check to see if both selected MOCs are of the same type.
//  *   have_space: a simple check to see if one of the Mocs is a space MOC.
//  *   have_time: a simple check to see if one of the Mocs is a time MOC.
fn files_have_stmoc(l: usize, r: usize) -> bool {
    matches!(U64MocStore.get_qty_type(l), Ok(MocQType::TimeSpace))
        || matches!(U64MocStore.get_qty_type(r), Ok(MocQType::TimeSpace))
}
fn files_have_same_type(l: usize, r: usize) -> bool {
    let a = fmt_qty(U64MocStore.get_qty_type(l).unwrap());
    let b = fmt_qty(U64MocStore.get_qty_type(r).unwrap());
    a == b
}
fn have_space(l: usize, r: usize) -> bool {
    matches!(U64MocStore.get_qty_type(l), Ok(MocQType::Space))
        || matches!(U64MocStore.get_qty_type(r), Ok(MocQType::Space))
}
fn have_time(l: usize, r: usize) -> bool {
    matches!(U64MocStore.get_qty_type(l), Ok(MocQType::Time))
        || matches!(U64MocStore.get_qty_type(r), Ok(MocQType::Time))
}
