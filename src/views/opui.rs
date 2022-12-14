#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use core::fmt;

use crate::commons::*;
use crate::controllers::{op1::*, op2::*};
use crate::models::{store, store::get_store, store::list_mocs};

use eframe::egui;
use egui::Ui;

enum Op {
    One(Op1),
    Two(Op2),
}
impl Default for Op {
    fn default() -> Self {
        Op::One(Op1::Complement)
    }
}
impl PartialEq for Op {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Op::One(a), Op::One(b)) => a.eq(b),
            (Op::Two(a), Op::Two(b)) => a.eq(b),
            _ => false,
        }
    }
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::One(o) => write!(f, "{}", o),
            Op::Two(t) => write!(f, "{}", t),
        }
    }
}

#[derive(Default)]
pub struct OpUis {
    name: String,
    operation: Op,
    deg: u8,
    picked_file: Option<String>,
    picked_second_file: Option<String>,
}
impl OpUis {
    // #Definition
    //      A function handling operations on a stored MOC.
    // #Args
    //  *   `ui`: The ui from the app.
    fn op_one_ui(&mut self, ui: &mut Ui) {
        // An operation combo box including Intersection and Union.
        let sel_text = format!("{}", self.operation);

        ui.label("Operation :");
        egui::ComboBox::from_id_source("Operation_cbox")
            .selected_text(sel_text)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.operation, Op::One(Op1::Complement), "Complement");
                ui.selectable_value(
                    &mut self.operation,
                    Op::One(Op1::Degrade { new_depth: 0 }),
                    "Degrade",
                );
                if self.picked_file.is_some()
                    && store::get_qty(&self.picked_file.clone().unwrap()) == Ok(Qty::Space)
                {
                    ui.selectable_value(&mut self.operation, Op::One(Op1::Extend), "Extend");
                    ui.selectable_value(&mut self.operation, Op::One(Op1::Contract), "Contract");
                    ui.selectable_value(&mut self.operation, Op::One(Op1::ExtBorder), "ExtBorder");
                    ui.selectable_value(&mut self.operation, Op::One(Op1::IntBorder), "IntBorder");
                    ui.selectable_value(&mut self.operation, Op::One(Op1::Split), "Split");
                    ui.selectable_value(
                        &mut self.operation,
                        Op::One(Op1::SplitIndirect),
                        "SplitIndirect",
                    );
                }
            });
    }

    // #Definition
    //      A function handling operations on a stored MOC.
    // #Args
    //  *   `ui`: The ui from the app.
    fn op_two_ui(&mut self, ui: &mut Ui) {
        // An operation combo box including Intersection and Union
        if self.picked_file.is_some() && self.picked_second_file.is_some() {
            if self.files_have_same_type() {
                if self.operation.eq(&Op::Two(Op2::SFold))
                    || self.operation.eq(&Op::Two(Op2::TFold))
                {
                    self.operation = Op::Two(Op2::Intersection);
                }
                let sel_text = format!("{}", self.operation);

                ui.label("Operation :");
                egui::ComboBox::from_id_source("Operation_cbox")
                    .selected_text(sel_text)
                    .show_ui(ui, |ui| {
                        if self.files_have_same_type() {
                            ui.selectable_value(
                                &mut self.operation,
                                Op::Two(Op2::Intersection),
                                "Intersection",
                            );
                            ui.selectable_value(&mut self.operation, Op::Two(Op2::Minus), "Minus");
                            ui.selectable_value(&mut self.operation, Op::Two(Op2::Union), "Union");
                            ui.selectable_value(
                                &mut self.operation,
                                Op::Two(Op2::Difference),
                                "Difference",
                            );
                        }
                    });
            } else if self.files_have_stmoc() {
                ui.horizontal(|ui| {
                    if store::get_qty(&self.picked_file.clone().unwrap()) == Ok(Qty::Space)
                        || store::get_qty(&self.picked_second_file.clone().unwrap())
                            == Ok(Qty::Space)
                    {
                        ui.label("Operation:");
                        ui.add_enabled(false, egui::widgets::Button::new("SFold"));
                        ui.end_row();
                        self.operation = Op::Two(Op2::SFold);
                    } else if store::get_qty(&self.picked_file.clone().unwrap()) == Ok(Qty::Time)
                        || store::get_qty(&self.picked_second_file.clone().unwrap())
                            == Ok(Qty::Time)
                    {
                        ui.label("Operation:");
                        ui.add_enabled(false, egui::widgets::Button::new("TFold"));
                        ui.end_row();
                        self.operation = Op::Two(Op2::TFold);
                    }
                });
            } else {
                ui.label(
                    "Files need to be of same type or Space or Time and Timespace (for folds)",
                );
            }
        } else {
            ui.label("Pick files on which to do operation");
        }
    }

    // #Definition
    //      A function creating the UI for operations on a stored MOC.
    // #Args
    //  *   `ui`: The ui from the app.
    //  *   `e`: an optional String in case of past errors to keep it visible until change
    pub(crate) fn moc_op1(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();

        let mut op: Op1 = Op1::Complement;
        if let Op::One(o) = self.operation {
            op = o;
        }

        //If no file has been imported yet
        if list_mocs().unwrap().is_empty() {
            ui.label("Pick a file!");
        //If files have been imported and can be chosen from
        } else {
            //Defaults to "pick one" before leaving the user choose which moc he wants to operate on
            let mut sel_text = "pick one".to_string();
            if self.picked_file.is_some() {
                sel_text = self.picked_file.clone().unwrap();
            }

            // The small paragraph before the match sets a grid layout to have every element aligned
            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    //Combo box containing the different files that can be picked from
                    ui.label("MOC : ");
                    self.make_cbox(ui, sel_text.as_str(), "file_cbox", None);
                    ui.end_row();

                    self.op_one_ui(ui);
                    ui.end_row();

                    //In case of degrade option ask for new depth
                    let deg = matches!(self.operation, Op::One(Op1::Degrade { new_depth: _ }));
                    if deg {
                        ui.add(egui::Slider::new(&mut self.deg, 0..=25));
                        ui.end_row();
                    }

                    if self.picked_file.is_some() {
                        ui.label("New MOC name :");
                        ui.text_edit_singleline(&mut self.name);
                        ui.end_row();

                        //Button launching the operation
                        if store::get_qty(&self.picked_file.clone().unwrap()) != Ok(Qty::Timespace)
                        {
                            if ui.button("Launch").clicked() {
                                err = None;
                                if deg {
                                    op = Op1::Degrade {
                                        new_depth: self.deg,
                                    }
                                }
                                let moc = self.picked_file.clone().unwrap();

                                if self.name.is_empty() {
                                    self.name = format!("{}_{}", op, moc);
                                }
                                let _ = op1(&moc, op, &self.name).map_err(|e| err = Some(e));
                                self.name = String::default();
                            };
                        } else {
                            ui.label("SpaceTime MOCs cannot be operated on alone.");
                        }
                    }
                });
        }
        err
    }

    // #Definition
    //      A function creating the UI for operations on 2 stored MOCs.
    // #Args
    //  *   `ui`: The ui from the app.
    //  *   `e`: an optional String in case of past errors to keep it visible until change
    pub(crate) fn moc_op2(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();

        let mut op: Op2 = Op2::Intersection;
        if let Op::Two(t) = self.operation {
            op = t;
        }

        // If no file has been imported yet.
        if list_mocs().unwrap().len() < 2 {
            ui.label("Pick at least 2 files!");
        // If files have been imported and can be chosen from.
        } else {
            let mut sel_text = "pick one".to_string();
            let mut sel_text_2 = "pick one".to_string();
            if self.picked_file.is_some() {
                sel_text = self.picked_file.clone().unwrap();
            }
            if self.picked_second_file.is_some() {
                sel_text_2 = self.picked_second_file.clone().unwrap();
            }

            // The small paragraph before the match sets a grid layout to have every element aligned.
            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
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

                    if (self.picked_file.is_some() && self.picked_second_file.is_some())
                        && (self.files_have_same_type() || self.files_have_stmoc())
                    {
                        ui.label("New MOC name :");
                        ui.text_edit_singleline(&mut self.name);
                        ui.end_row();

                        //Button launching the operation
                        if ui.button("Launch").clicked() {
                            let mut l = self.picked_file.as_ref().unwrap();
                            let mut r = self.picked_second_file.as_ref().unwrap();
                            if store::get_qty(l) == Ok(Qty::Timespace) {
                                std::mem::swap(&mut r, &mut l);
                            }
                            if self.name.is_empty() {
                                self.name = format!("{}_{}_{}", op, l, r);
                            }
                            let _ = op2(l, r, op, &self.name).map_err(|e| err = Some(e));
                            self.name = String::default();
                        };
                    }
                });
        }
        err
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
                        ui.selectable_value(
                            &mut self.picked_file,
                            Some(file.0.to_string()),
                            file.0,
                        );
                    } else {
                        ui.selectable_value(
                            &mut self.picked_second_file,
                            Some(file.0.to_string()),
                            file.0,
                        );
                    }
                }
            });
    }
    // #Definitions
    //  *   files_have_stmoc: a simple check to see if a space time MOC is present in the 2 selected MOCs.
    //  *   files_have_same_type: a simple check to see if both selected MOCs are of the same type.
    fn files_have_stmoc(&mut self) -> bool {
        store::get_qty(&self.picked_second_file.clone().unwrap()) == Ok(Qty::Timespace)
            || store::get_qty(&self.picked_file.clone().unwrap()) == Ok(Qty::Timespace)
    }
    fn files_have_same_type(&mut self) -> bool {
        store::get_qty(&self.picked_file.clone().unwrap())
            .eq(&store::get_qty(&self.picked_second_file.clone().unwrap()))
    }
}
