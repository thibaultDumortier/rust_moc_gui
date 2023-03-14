use crate::controllers::op1::*;
use crate::utils::namestore::{get_last, get_name, get_store, list_names};

use eframe::egui;
use egui::{TextEdit, Ui};
use moc::storage::u64idx::common::MocQType;
use moc::storage::u64idx::U64MocStore;

use super::{SubUi, View};

#[derive(Default, Clone, Eq, PartialEq)]
pub struct UnitaryUi {
    name: String,
    operation: Op1,
    deg: u8,
    picked_file: Option<usize>,
}

impl SubUi for UnitaryUi {
    fn name(&self) -> &'static str {
        "MOC unitary logical operation"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(false)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl View for UnitaryUi {
    // #Definition
    //      A function creating the UI for operations on a stored MOC.
    // #Args
    //  *   `ui`: The ui from the app.
    //  *   `e`: an optional String in case of past errors to keep it visible until change
    fn ui(&mut self, ui: &mut Ui) {
        let mut op: Op1 = self.operation;

        //If no file has been imported yet
        if list_names().unwrap().is_empty() {
            ui.label("Pick a file!");
        //If files have been imported and can be chosen from
        } else {
            //Defaults to last loaded MOC before leaving the user choose which moc he wants to operate on
            let sel_text: String;
            if self.picked_file.is_some() {
                if let Ok(txt) = get_name(self.picked_file.unwrap()).map_err(|e| return e) {
                    sel_text = txt
                } else {
                    self.picked_file = Some(get_last(0).unwrap().0);
                    sel_text = get_name(self.picked_file.unwrap())
                        .map_err(|e| return e)
                        .unwrap();
                }
            } else {
                self.picked_file = Some(get_last(0).unwrap().0);
                sel_text = get_name(self.picked_file.unwrap())
                    .map_err(|e| return e)
                    .unwrap();
            }

            // The small paragraph before the match sets a grid layout to have every element aligned
            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([5.0, 4.0])
                .striped(false)
                .show(ui, |ui| {
                    //Combo box containing the different files that can be picked from
                    ui.label("MOC : ");
                    self.make_cbox(ui, sel_text.as_str());
                    ui.end_row();

                    self.op_one_ui(ui);
                    ui.end_row();

                    //In case of degrade option ask for new depth
                    let deg = matches!(self.operation, Op1::Degrade { new_depth: _ });
                    if deg {
                        ui.label("Depth : ");
                        ui.add(egui::Slider::new(&mut self.deg, 0..=25));
                        ui.end_row();
                    }

                    if self.picked_file.is_some() {
                        ui.label("New MOC name :");
                        ui.add(TextEdit::singleline(&mut self.name).hint_text("Name"));
                        ui.end_row();

                        //Button launching the operation
                        if !matches!(
                            U64MocStore.get_qty_type(self.picked_file.unwrap()),
                            Ok(MocQType::TimeSpace)
                        ) {
                            if ui.button("Launch").clicked() {
                                if deg {
                                    op = Op1::Degrade {
                                        new_depth: self.deg,
                                    }
                                }

                                if self.name.is_empty() {
                                    self.name = format!(
                                        "{}_{}",
                                        op,
                                        get_name(self.picked_file.unwrap()).unwrap()
                                    );
                                }
                                let _ = op1(self.picked_file.unwrap(), op, &self.name)
                                    .map_err(|e| return e);
                                self.name = String::default();
                            };
                        } else {
                            ui.label("SpaceTime MOCs cannot be operated on alone.");
                        }
                    }
                });
        }
    }
}

impl UnitaryUi {
    // #Definition
    //      A function that creates comboboxes.
    // #Args
    //  *   ui: Ui, the ui from the app.
    //  *   sel_text: &str, the text to show in the combo box.
    //  *   id: &str, the combobox gui ID.
    fn make_cbox(&mut self, ui: &mut Ui, sel_text: &str) {
        egui::ComboBox::from_id_source("file_cbox")
            .selected_text(sel_text)
            .show_ui(ui, |ui| {
                for file in get_store().read().unwrap().iter() {
                    ui.selectable_value(&mut self.picked_file, Some(*file.0), &file.1 .0);
                }
            });
    }

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
                ui.selectable_value(&mut self.operation, Op1::Complement, "Complement");
                ui.selectable_value(
                    &mut self.operation,
                    Op1::Degrade { new_depth: 0 },
                    "Degrade",
                );
                if self.picked_file.is_some()
                    && matches!(
                        U64MocStore.get_qty_type(self.picked_file.unwrap()),
                        Ok(MocQType::Space)
                    )
                {
                    ui.selectable_value(&mut self.operation, Op1::Extend, "Extend");
                    ui.selectable_value(&mut self.operation, Op1::Contract, "Contract");
                    ui.selectable_value(&mut self.operation, Op1::ExtBorder, "ExtBorder");
                    ui.selectable_value(&mut self.operation, Op1::IntBorder, "IntBorder");
                    ui.selectable_value(&mut self.operation, Op1::Split, "Split");
                    ui.selectable_value(&mut self.operation, Op1::SplitIndirect, "SplitIndirect");
                }
            });
    }
}
