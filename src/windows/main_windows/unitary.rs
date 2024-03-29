use crate::controllers::op1::*;
use crate::utils::commons::err;
use crate::utils::namestore::{get_last, get_name, get_store, list_names};

use eframe::egui;
use egui::{TextEdit, Ui};
use moc::storage::u64idx::common::MocQType;
use moc::storage::u64idx::U64MocStore;

use crate::windows::{View, Window};

#[derive(Default, Clone, Eq, PartialEq)]
pub struct UnitaryUi {
    name: String,
    operation: Op1,
    deg: u8,
    picked_file: Option<usize>,
}

impl Window for UnitaryUi {
    fn name(&self) -> &'static str {
        "MOC unitary logical operation"
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
            // The small paragraph before the match sets a grid layout to have every element aligned
            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([5.0, 4.0])
                .striped(false)
                .show(ui, |ui| {
                    //Combo box containing the different files that can be picked from
                    ui.label("MOC : ");
                    if self.picked_file.is_some() && get_name(self.picked_file.unwrap()).is_ok() {
                        self.make_cbox(ui, get_name(self.picked_file.unwrap()).unwrap());
                    } else {
                        self.picked_file = Some(get_last().unwrap().0);
                        self.make_cbox(ui, "pick a file".to_owned());
                    }
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
                                    .map_err(|e| err(&e));
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
    fn make_cbox(&mut self, ui: &mut Ui, text: String) {
        egui::ComboBox::from_id_source("file_cbox")
            .selected_text(text)
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

// #Definition
//      a view showing buttons of every possible unitary MOC operation when right clicking
// #Args
//  *   `id`: the index of the MOC to be operated on
//  *   `ui`: the ui used by the app
pub(crate) fn lite_unit_ui(ui: &mut Ui, id: usize) {
    if !matches!(U64MocStore.get_qty_type(id), Ok(MocQType::TimeSpace)) {
        if ui.button("Complement").clicked() {
            lite_op(id, Op1::Complement);
        };
        if matches!(U64MocStore.get_qty_type(id), Ok(MocQType::Space)) {
            if ui.button("Extend").clicked() {
                lite_op(id, Op1::Extend);
            };
            if ui.button("Contract").clicked() {
                lite_op(id, Op1::Contract);
            };
            if ui.button("ExtBorder").clicked() {
                lite_op(id, Op1::ExtBorder);
            };
            if ui.button("IntBorder").clicked() {
                lite_op(id, Op1::IntBorder);
            };
            if ui.button("Split").clicked() {
                lite_op(id, Op1::Split);
            };
            if ui.button("SplitIndirect").clicked() {
                lite_op(id, Op1::SplitIndirect);
            };
        }
    } else {
        ui.label("SpaceTime MOCs cannot be operated on alone.");
    }
}
// #Definition
//      the button launching the operation
// #Args
//  *   `id`: the index of the MOC to be operated on
//  *   `operation`: the operation that needs to be applied on the MOCs
// #Errors
//      may show an error message coming from the op1 function
fn lite_op(id: usize, operation: Op1) {
    //Button launching the operation
    let name = format!("{}_{}", operation, get_name(id).unwrap());
    let _ = op1(id, operation, &name).map_err(|e| err(&e));
}
