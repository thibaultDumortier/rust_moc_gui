#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::commons::*;
use crate::op1::*;
use crate::op2::*;
use crate::store;
use crate::store::get_store;
use crate::store::list_mocs;

use eframe::egui;
use egui::menu;
use egui::Ui;
use egui_extras::{Size, TableBuilder};
use wasm_bindgen::prelude::wasm_bindgen;

//Import javascript log function
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

//An operation enumerator
enum Op {
    One(Op1),
    Two(Op2),
    Opnone,
}
impl Default for Op {
    fn default() -> Self {
        Op::One(Op1::default())
    }
}
impl PartialEq for Op {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Op::One(a), Op::One(b)) => a.eq(b),
            (Op::Two(a), Op::Two(b)) => a.eq(b),
            (Op::Opnone, Op::Opnone) => true,
            _ => false,
        }
    }
}

//FileApp struct
#[derive(Default)]
pub struct FileApp {
    picked_file: Option<String>,
    picked_second_file: Option<String>,
    operation: Op,
    deg: u8,
    error: Option<String>,
    name: String,
}
impl eframe::App for FileApp {
    /*
        update: function of FileApp struct from eframe::App
        Description: A function updating the state of the application
        Parameters:
            ctx: &equi::Context, the app's context
            frame is unused but mandatory
        Returns: ()
    */
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
                ui.selectable_value(&mut self.operation, Op::Opnone, "MOC list");
                ui.selectable_value(
                    &mut self.operation,
                    Op::One(Op1::default()),
                    "1 MOC operation",
                );
                ui.selectable_value(
                    &mut self.operation,
                    Op::Two(Op2::default()),
                    "2 MOCs operation",
                );
            });
            ui.end_row();

            ui.separator();
            match &self.operation {
                Op::One(o) => self.op_one_ui(ui, o.clone()),
                Op::Two(t) => self.op_two_ui(ui, t.clone()),
                Op::Opnone => self.list_ui(ui),
            }
        });
    }
}
impl FileApp {
    /*
        new: function of FileApp struct
        Description: A function handling the contents of the top bar
        Parameters: None
        Returns: FileApp
    */
    pub fn new() -> Self {
        FileApp {
            picked_file: None,
            picked_second_file: None,
            operation: Op::default(),
            deg: 0,
            error: None,
            name: String::default(),
        }
    }

    /*
        bar_contents: function of FileApp struct
        Description: A function handling the contents of the top bar
        Parameters:
            ui: Ui, the ui from the app
        Returns: ()
    */
    fn bar_contents(&mut self, ui: &mut Ui) {
        egui::widgets::global_dark_light_mode_switch(ui);

        ui.separator();

        menu::bar(ui, |ui| {
            ui.menu_button("Files", |ui| {
                ui.menu_button("Load", |ui| {
                    if ui.button("FITS").clicked() {
                        match load(&["fits"], Qty::Space) {
                            Ok(_) => (),
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
            ui.label(self.error.as_ref().unwrap());
        }
    }

    /*
        op_one_ui: function of FileApp struct
        Description: A function handling operations on a moc launched by the app
        Parameters:
            ui: Ui, the ui from the app
            operation: Op1, operation enumerator of the selected operation
        Returns: ()
    */
    fn op_one_ui(&mut self, ui: &mut Ui, operation: Op1) {
        // An operation combo box including Intersection and Union
        let sel_text = format!("{}", operation);

        ui.horizontal(|ui| {
            ui.label("Operation :");
            egui::ComboBox::from_id_source("Operation_cbox")
                .selected_text(sel_text)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.operation,
                        Op::One(Op1::Complement),
                        "Complement",
                    );
                    ui.selectable_value(
                        &mut self.operation,
                        Op::One(Op1::Degrade { new_depth: 0 }),
                        "Degrade",
                    );
                    ui.selectable_value(&mut self.operation, Op::One(Op1::Extend), "Extend");
                    ui.selectable_value(&mut self.operation, Op::One(Op1::Contract), "Contract");
                    ui.selectable_value(&mut self.operation, Op::One(Op1::ExtBorder), "ExtBorder");
                    ui.selectable_value(&mut self.operation, Op::One(Op1::IntBorder), "IntBorder");
                    if self.picked_file.is_some() {
                        if store::get_qty(&self.picked_file.clone().unwrap()) == Ok(Qty::Space) {
                            ui.selectable_value(&mut self.operation, Op::One(Op1::Split), "Split");
                            ui.selectable_value(
                                &mut self.operation,
                                Op::One(Op1::SplitIndirect),
                                "SplitIndirect",
                            );
                        }
                    }
                });
        });

        //A file choosing combobox
        match operation {
            Op1::Complement => self.moc_op1(ui, Op1::Complement),
            Op1::Degrade { new_depth } => self.moc_op1(ui, Op1::Degrade { new_depth }),
            Op1::Extend => self.moc_op1(ui, Op1::Extend),
            Op1::Contract => self.moc_op1(ui, Op1::Contract),
            Op1::ExtBorder => self.moc_op1(ui, Op1::ExtBorder),
            Op1::IntBorder => self.moc_op1(ui, Op1::IntBorder),
            Op1::Split => self.moc_op1(ui, Op1::Split),
            Op1::SplitIndirect => self.moc_op1(ui, Op1::SplitIndirect),
        }
    }

    /*
        op_two_ui: function of FileApp struct
        Description: A function handling operations on 2 mocs launched by the app
        Parameters:
            ui: Ui, the ui from the app
            operation: Op2, operation enumerator of the selected operation
        Returns: ()
    */
    fn op_two_ui(&mut self, ui: &mut Ui, operation: Op2) {
        // An operation combo box including Intersection and Union
        if self.picked_file.is_some() && self.picked_second_file.is_some() {
            if store::get_qty(&self.picked_file.clone().unwrap())
                .eq(&store::get_qty(&self.picked_second_file.clone().unwrap()))
            {
                let sel_text = format!("{}", operation);

                ui.horizontal(|ui| {
                    ui.label("Operation :");
                    egui::ComboBox::from_id_source("Operation_cbox")
                        .selected_text(sel_text)
                        .show_ui(ui, |ui| {
                            if store::get_qty(&self.picked_file.clone().unwrap())
                                .eq(&store::get_qty(&self.picked_second_file.clone().unwrap()))
                            {
                                ui.selectable_value(
                                    &mut self.operation,
                                    Op::Two(Op2::Intersection),
                                    "Intersection",
                                );
                                ui.selectable_value(
                                    &mut self.operation,
                                    Op::Two(Op2::Minus),
                                    "Minus",
                                );
                                ui.selectable_value(
                                    &mut self.operation,
                                    Op::Two(Op2::Union),
                                    "Union",
                                );
                                ui.selectable_value(
                                    &mut self.operation,
                                    Op::Two(Op2::Difference),
                                    "Difference",
                                );
                            }
                        });
                });
            } else if store::get_qty(&self.picked_file.clone().unwrap()) == Ok(Qty::Timespace)
                || store::get_qty(&self.picked_second_file.clone().unwrap()) == Ok(Qty::Timespace)
            {
                let sel_text = format!("{}", operation);
                ui.horizontal(|ui| {
                    ui.label("Operation :");
                    egui::ComboBox::from_id_source("Operation_cbox")
                        .selected_text(sel_text)
                        .show_ui(ui, |ui| {
                            if store::get_qty(&self.picked_file.clone().unwrap()) == Ok(Qty::Space)
                                || store::get_qty(&self.picked_second_file.clone().unwrap())
                                    == Ok(Qty::Space)
                            {
                                ui.selectable_value(
                                    &mut self.operation,
                                    Op::Two(Op2::SFold),
                                    "SFold",
                                );
                            } else if store::get_qty(&self.picked_file.clone().unwrap())
                                == Ok(Qty::Time)
                                || store::get_qty(&self.picked_second_file.clone().unwrap())
                                    == Ok(Qty::Time)
                            {
                                ui.selectable_value(
                                    &mut self.operation,
                                    Op::Two(Op2::TFold),
                                    "TFold",
                                );
                            }
                        });
                });
            } else {
                ui.label(
                    "Files need to be of same type or TimeSpace and Space or Time (for folds)",
                );
            }
        } else {
            ui.label("Pick files on which to do operation");
        }

        //A file choosing combobox
        match operation {
            Op2::Intersection => self.moc_op2(ui, Op2::Intersection),
            Op2::Union => self.moc_op2(ui, Op2::Union),
            Op2::Difference => self.moc_op2(ui, Op2::Difference),
            Op2::Minus => self.moc_op2(ui, Op2::Minus),
            Op2::TFold => self.moc_op2(ui, Op2::TFold),
            Op2::SFold => self.moc_op2(ui, Op2::SFold),
        }
    }

    /*
        moc_op1: function of FileApp struct
        Description: A function handling operations on a moc launched by the app
        Parameters:
            ui: Ui, the ui from the app
            op1: Op1, operation enumerator of the selected operation
        Returns: ()
    */
    fn moc_op1(&mut self, ui: &mut Ui, mut op: Op1) {
        //If no file has been imported yet
        if list_mocs().unwrap().len() == 0 {
            ui.label("Pick a file!");
        //If files have been imported and can be chosen from
        } else {
            //Defaults to "pick one" before leaving the user choose which moc he wants to operate on
            let mut sel_text = "pick one".to_string();
            if self.picked_file.is_some() {
                sel_text = self.picked_file.clone().unwrap();
            }

            //In case of degrade option ask for new depth
            let deg = matches!(op, Op1::Degrade { new_depth: _ });
            if deg {
                ui.add(egui::Slider::new(&mut self.deg, 0..=25));
            }

            //Combo box containing the different files that can be picked from
            ui.horizontal(|ui| {
                ui.label("MOC : ");
                self.make_cbox(ui, sel_text.as_str(), "file_cbox", None);
            });

            ui.horizontal(|ui| {
                ui.label("New MOC name :");
                ui.text_edit_singleline(&mut self.name);
            });

            //Button launching the operation
            ui.horizontal(|ui| {
                if ui.button("Launch").clicked() {
                    if deg {
                        op = Op1::Degrade {
                            new_depth: self.deg,
                        }
                    }
                    let moc = self.picked_file.clone().unwrap();

                    if self.name.len() == 0 {
                        if !op1(&moc, op, &format!("{}_{}", op.to_string(), moc)).is_ok() {
                            ui.label("Error when trying to do operation");
                        }
                    } else {
                        if !op1(&moc, op, &self.name).is_ok() {
                            ui.label("Error when trying to do operation");
                            self.name = String::default();
                        }
                    }
                };
            });
        }
    }

    /*
        moc_op2: function of FileApp struct
        Description: A function handling operations on 2 mocs launched by the app
        Parameters:
            ui: Ui, the ui from the app
            op2: Op2, operation enumerator of the selected operation
        Returns: ()
    */
    fn moc_op2(&mut self, ui: &mut Ui, op: Op2) {
        //If no file has been imported yet
        if list_mocs().unwrap().len() < 2 {
            ui.label("Pick at least 2 files!");
        //If files have been imported and can be chosen from
        } else {
            let mut sel_text = "pick one".to_string();
            let mut sel_text_2 = "pick one".to_string();
            if self.picked_file.is_some() {
                sel_text = self.picked_file.clone().unwrap();
            }
            if self.picked_second_file.is_some() {
                sel_text_2 = self.picked_second_file.clone().unwrap();
            }
            //Combo boxes containing the different files that can be picked from
            ui.horizontal(|ui| {
                ui.label("First MOC :");
                self.make_cbox(ui, &sel_text, "file_cbox", None);
            });
            ui.horizontal(|ui| {
                ui.label("Second MOC :");
                self.make_cbox(ui, &sel_text_2, "file_cbox_2", Some(1));
            });

            ui.horizontal(|ui| {
                ui.label("New MOC name :");
                ui.text_edit_singleline(&mut self.name);
            });

            //Button launching the operation
            ui.horizontal(|ui| {
                if ui.button("Launch").clicked() {
                    let l = self.picked_file.as_ref().unwrap();
                    let r = self.picked_second_file.as_ref().unwrap();
                    if self.name.len() == 0 {
                        if !op2(&l, &r, op, &format!("{}_{}_{}", op.to_string(), l, r)).is_ok() {
                            ui.label("Error when trying to do operation");
                        }
                    } else {
                        if !op2(&l, &r, op, &self.name).is_ok() {
                            ui.label("Error when trying to do operation");
                            self.name = String::default();
                        }
                    }
                };
            });
        }
    }

    fn list_ui(&mut self, ui: &mut Ui) {
        let mut filenames: Vec<String> = Vec::default();
        for file in get_store().read().unwrap().iter() {
            filenames.push(file.0.to_string());
        }
        let txt_h = 30.0;
        ui.vertical(|ui| {
            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Size::initial(300.0).at_least(100.0))
                .column(Size::initial(20.0).at_least(20.0))
                .column(Size::remainder().at_least(20.0))
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
                            ui.label(filenames.get(row_index).unwrap());
                        });
                        row.col(|ui| {
                            ui.menu_button("📥", |ui| {
                                if ui.button("FITS").clicked() {
                                    if !to_fits_file(filenames.get(row_index).unwrap()).is_ok() {
                                        ui.label("Error when trying to create file");
                                    }
                                }
                                if ui.button("ASCII").clicked() {
                                    if !to_ascii_file(filenames.get(row_index).unwrap(), Some(0))
                                        .is_ok()
                                    {
                                        ui.label("Error when trying to create file");
                                    }
                                }
                                if ui.button("JSON").clicked() {
                                    if !to_json_file(filenames.get(row_index).unwrap(), Some(0))
                                        .is_ok()
                                    {
                                        ui.label("Error when trying to create file");
                                    }
                                }
                            });
                        });
                        row.col(|ui| {
                            if ui.button("❌").clicked() {
                                if !store::drop(filenames.get(row_index).unwrap()).is_ok() {
                                    ui.label("Error when trying to remove file");
                                }
                            }
                        });
                    })
                })
        });
    }

    /*
        make_cbox: function of FileApp struct
        Description: A function creating comboboxes
        Parameters:
            ui: Ui, the ui from the app
            sel_text: &str, the text to show in the combo box
            id: &str, the combobox gui ID
            op: Option<u8>, to know if there needs to be multiple selected mocs.
        Returns: ()
    */
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
}
