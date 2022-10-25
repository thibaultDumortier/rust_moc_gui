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
    writing: MocWType,
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
            writing: MocWType::default(),
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
                        assert!(load(&["fits"], Qty::Space).is_ok());
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
                    ui.selectable_value(&mut self.operation, Op::One(Op1::Split), "Split");
                    ui.selectable_value(
                        &mut self.operation,
                        Op::One(Op1::SplitIndirect),
                        "SplitIndirect",
                    );
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
        let sel_text = format!("{}", operation);
        ui.horizontal(|ui| {
            ui.label("Operation :");
            egui::ComboBox::from_id_source("Operation_cbox")
                .selected_text(sel_text)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.operation,
                        Op::Two(Op2::Intersection),
                        "Intersection",
                    );
                    ui.selectable_value(&mut self.operation, Op::Two(Op2::Union), "Union");
                    ui.selectable_value(
                        &mut self.operation,
                        Op::Two(Op2::Difference),
                        "Difference",
                    );
                    ui.selectable_value(&mut self.operation, Op::Two(Op2::Minus), "Minus");
                    ui.selectable_value(&mut self.operation, Op::Two(Op2::TFold), "TFold");
                    ui.selectable_value(&mut self.operation, Op::Two(Op2::SFold), "SFold");
                });
        });

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
        if list_mocs().unwrap().length() == 0 {
            ui.label("Pick a file!");
        //If files have been imported and can be chosen from
        } else {
            //Defaults to "pick one" before leaving the user choose which moc he wants to operate on
            let mut sel_text = "pick one".to_string();
            if self.picked_file.is_some() {
                sel_text = self.picked_file.clone().unwrap();
            }
            //Combo box containing the different files that can be picked from
            ui.horizontal(|ui| {
                ui.label("Moc : ");
                self.make_cbox(ui, sel_text.as_str(), "file_cbox", None);
            });

            //In case of degrade option ask for new depth
            let deg = matches!(op, Op1::Degrade { new_depth: _ });
            if deg {
                ui.add(egui::Slider::new(&mut self.deg, 0..=25));
            }
            //Button launching the operation
            ui.horizontal(|ui| {
                self.ui_writing_type(ui);
                if ui.button("Launch").clicked() {
                    if deg {
                        op = Op1::Degrade {
                            new_depth: self.deg,
                        }
                    }
                    let moc = self.picked_file.clone().unwrap();
                    if !op1(&moc, op, "result").is_ok() {
                        ui.label("Error when trying to do operation");
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
        if list_mocs().unwrap().length() < 2 {
            ui.label("Pick at least 2 files!");
        //If files have been imported and can be chosen from
        } else {
            //If no file has been imported yet
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
                ui.label("First moc :");
                self.make_cbox(ui, &sel_text, "file_cbox", None);
            });
            ui.horizontal(|ui| {
                ui.label("Second moc :");
                self.make_cbox(ui, &sel_text_2, "file_cbox_2", Some(1));
            });

            //Button launching the operation
            ui.horizontal(|ui| {
                self.ui_writing_type(ui);
                if ui.button("Launch").clicked() {
                    let l = self.picked_file.as_ref().unwrap();
                    let r = self.picked_second_file.as_ref().unwrap();
                    if op2(&l, &r, op, "result").is_ok() {
                        ui.label("Error when trying to do operation");
                    }
                };
            });
        }
    }

    fn list_ui(&mut self, ui: &mut Ui) {
        let mut filenames: Vec<String> = Vec::default();
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                for file in get_store().read().unwrap().iter() {
                    filenames.push(file.0.to_string());
                    ui.label(file.0);
                }
            });
            ui.vertical(|ui| {
                for filen in filenames.iter() {
                    ui.horizontal(|ui| {
                        if ui.button("remove").clicked() {
                            if !store::drop(filen).is_ok() {
                                ui.label("Error when trying to remove file");
                            }
                        }
                        if ui.button("FITS").clicked() {
                            to_fits_file(filen);
                        }
                        if ui.button("ASCII").clicked() {
                            to_ascii_file(filen, Some(0));
                        }
                        if ui.button("JSON").clicked() {
                            to_json_file(filen, Some(0));
                        }
                    });
                }
            });
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

    /*
        ui_writing_type: function of FileApp struct
        Description: A function that creates a simple combobox to select the type of output
        Parameters:
            ui: Ui, the ui from the app
        Returns: ()
    */
    fn ui_writing_type(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_id_source("writing_cbox")
            .selected_text(self.writing.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.writing,
                    MocWType::Fits,
                    MocWType::Fits.to_string(),
                );
                ui.selectable_value(
                    &mut self.writing,
                    MocWType::Json,
                    MocWType::Json.to_string(),
                );
                ui.selectable_value(
                    &mut self.writing,
                    MocWType::Ascii,
                    MocWType::Ascii.to_string(),
                );
            });
    }
}
