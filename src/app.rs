#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::commons::*;
use crate::op1::*;
use crate::op2::*;

use eframe::egui;
use egui::Ui;
use moc::deser::fits::{from_fits_ivoa, MocIdxType};
use rfd::AsyncFileDialog;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

//Import javascript log function
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

//A file like object containing the name of the file and the data from the moc
#[derive(Default, Clone)]
pub struct UploadedFiles {
    name: String,
    data: Option<InternalMoc>,
}
impl PartialEq for UploadedFiles {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

enum Op {
    One(Op1),
    Two(Op2),
}
impl Default for Op {
    fn default() -> Self {
        Op::One(Op1::default())
    }
}
impl PartialEq for Op {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Op::One(_), Op::One(_)) => true,
            (Op::One(_), Op::Two(_)) => false,
            (Op::Two(_), Op::One(_)) => false,
            (Op::Two(_), Op::Two(_)) => true,
        }
    }
}

//FileApp struct
/*
    * files: contains the different uploaded files
    *

*/
#[derive(Default)]
pub struct FileApp {
    files: Arc<Mutex<Vec<UploadedFiles>>>,
    picked_file: Option<UploadedFiles>,
    picked_second_file: Option<UploadedFiles>,
    operation: Op,
    deg: u8,
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
                ui.selectable_value(
                    &mut self.operation,
                    Op::One(Op1::default()),
                    "1 moc operation",
                );
                ui.selectable_value(
                    &mut self.operation,
                    Op::Two(Op2::default()),
                    "2 mocs operation",
                );
            });
            ui.end_row();

            match &self.operation {
                Op::One(o) => self.op_one_ui(ui, o.clone()),
                Op::Two(t) => self.op_two_ui(ui, t.clone()),
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
            files: Arc::new(Mutex::new(Default::default())),
            picked_file: None,
            picked_second_file: None,
            operation: Op::default(),
            deg: 0,
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

        if ui.button("Open file...").clicked() {
            assert!(self.fileclick().is_ok());
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
    pub fn moc_op1(&mut self, ui: &mut Ui, mut op1: Op1) {
        //If no file has been imported yet
        if self.files.lock().unwrap().to_vec().is_empty() {
            ui.label("Pick a file!");
        //If files have been imported and can be chosen from
        } else {
            //Defaults to "pick one" before leaving the user choose which moc he wants to operate on
            let mut sel_text = "pick one".to_string();
            if self.picked_file.is_some() {
                sel_text = format!("{}", self.picked_file.as_ref().unwrap().name);
            }
            //Combo box containing the different files that can be picked from
            self.make_cbox(ui, sel_text.as_str(), "file_cbox", None);

            //In case of degrade option ask for new depth
            let deg: bool;
            match op1 {
                Op1::Degrade { new_depth: _ } => deg = true,
                _ => deg = false,
            }
            if deg {
                ui.add(egui::Slider::new(&mut self.deg, 0..=25));
            }

            //Button launching the operation
            if ui.button("Do Operation").clicked() {
                if deg {
                    op1 = Op1::Degrade {
                        new_depth: self.deg,
                    }
                }
                let moc = self.picked_file.clone().unwrap().data.unwrap();
                let res = match moc {
                    InternalMoc::Space(moc) => {
                        op1.perform_op1_on_smoc(&moc).map(InternalMoc::Space)
                    }
                };
                log(&format!("{:?}", res.unwrap().to_fits()));
            };
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
    pub fn moc_op2(&mut self, ui: &mut Ui, op2: Op2) {
        //If no file has been imported yet
        if self.files.lock().unwrap().to_vec().is_empty() {
            ui.label("Pick at least 2 files!");
        //If files have been imported and can be chosen from
        } else {
            //If no file has been imported yet
            let mut sel_text = "pick one".to_string();
            let mut sel_text_2 = "pick one".to_string();
            if self.picked_file.is_some() {
                sel_text = format!("{}", self.picked_file.as_ref().unwrap().name);
            }
            if self.picked_second_file.is_some() {
                sel_text_2 = format!("{}", self.picked_second_file.as_ref().unwrap().name);
            }
            //Combo boxes containing the different files that can be picked from
            ui.horizontal(|ui| {
                self.make_cbox(ui, sel_text.as_str(), "file_cbox", None);
                self.make_cbox(ui, sel_text_2.as_str(), "file_cbox_2", Some(1));
            });
            //Button launching the operation
            if ui.button("Do Operation").clicked() {
                let l = self.picked_file.clone().unwrap().data.unwrap();
                let r = self.picked_second_file.clone().unwrap().data.unwrap();
                let res;
                res = match (l, r) {
                    (InternalMoc::Space(l), InternalMoc::Space(r)) => {
                        op2.perform_op2_on_smoc(&l, &r).map(InternalMoc::Space)
                    }
                };
                log(&format!("{:?}", res.unwrap().to_fits()));
            };
        }
    }

    /*
        fileclick: function of FileApp struct
        Description: A function handling the clicking of the "open file" button
        Parameters: None
        Returns: a simple result for Ok or Error
    */
    pub fn fileclick(&mut self) -> Result<(), &str> {
        let task = AsyncFileDialog::new()
            .add_filter("MOCs", &["fits", "ascii", "json", "txt"])
            .pick_files();
        let files_cpy = self.files.clone();

        Self::execute(async move {
            let handle = task.await;
            let mut files: Vec<UploadedFiles> = Default::default();

            if let Some(handle) = handle {
                // If you care about wasm support you just read() the file
                for path in handle {
                    let mut file = UploadedFiles::default();
                    //Reads name and adds it to be shown to user
                    let file_name = path.file_name();
                    file.name = file_name;
                    //Reads file contents and adds it to the data
                    let file_content = path.read().await;
                    file.data = Some(
                        match from_fits_ivoa(Cursor::new(file_content.as_ref()))
                            .map_err(|e| JsValue::from_str(&e.to_string()))
                            .unwrap()
                        {
                            MocIdxType::U16(moc) => from_fits(moc),
                            MocIdxType::U32(moc) => from_fits(moc),
                            MocIdxType::U64(moc) => from_fits(moc),
                        }
                        .map_err(|e| JsValue::from_str(&e.to_string()))
                        .unwrap(),
                    );
                    files.push(file);
                }
                *(files_cpy.lock().unwrap()) = files;
            }
        });
        Ok(())
    }
    fn execute<F: std::future::Future<Output = ()> + 'static>(f: F) {
        wasm_bindgen_futures::spawn_local(f);
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
        egui::ComboBox::from_id_source("Operation_cbox")
            .selected_text(sel_text)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.operation, Op::One(Op1::Complement), "Complement");
                ui.selectable_value(
                    &mut self.operation,
                    Op::One(Op1::Degrade { new_depth: 0 }),
                    "Degrade",
                );
                ui.selectable_value(&mut self.operation, Op::One(Op1::Extend), "Extend");
                ui.selectable_value(&mut self.operation, Op::One(Op1::Contract), "Contract");
                ui.selectable_value(&mut self.operation, Op::One(Op1::ExtBorder), "ExtBorder");
                ui.selectable_value(&mut self.operation, Op::One(Op1::IntBorder), "IntBorder");
            });

        //A file choosing combobox
        match operation {
            Op1::Complement => self.moc_op1(ui, Op1::Complement),
            Op1::Degrade { new_depth } => self.moc_op1(ui, Op1::Degrade { new_depth }),
            Op1::Extend => self.moc_op1(ui, Op1::Extend),
            Op1::Contract => self.moc_op1(ui, Op1::Contract),
            Op1::ExtBorder => self.moc_op1(ui, Op1::ExtBorder),
            Op1::IntBorder => self.moc_op1(ui, Op1::IntBorder),
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
        egui::ComboBox::from_id_source("Operation_cbox")
            .selected_text(sel_text)
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.operation,
                    Op::Two(Op2::Intersection),
                    "Intersection",
                );
                ui.selectable_value(&mut self.operation, Op::Two(Op2::Union), "Union");
                ui.selectable_value(&mut self.operation, Op::Two(Op2::Difference), "Difference");
                ui.selectable_value(&mut self.operation, Op::Two(Op2::Minus), "Minus");
                ui.selectable_value(&mut self.operation, Op::Two(Op2::TFold), "TFold");
                ui.selectable_value(&mut self.operation, Op::Two(Op2::SFold), "SFold");
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
        let files = self.files.lock().unwrap().to_vec();

        egui::ComboBox::from_id_source(id)
            .selected_text(sel_text)
            .show_ui(ui, |ui| {
                for file in &files {
                    if op.is_none() {
                        ui.selectable_value(
                            &mut self.picked_file,
                            Some(file.clone()),
                            file.clone().name,
                        );
                    } else {
                        ui.selectable_value(
                            &mut self.picked_second_file,
                            Some(file.clone()),
                            file.clone().name,
                        );
                    }
                }
            });
    }
}
