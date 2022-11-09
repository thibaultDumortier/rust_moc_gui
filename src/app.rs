#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::commons::*;
use crate::creation::*;
use crate::op1::*;
use crate::op2::*;
use crate::store;
use crate::store::get_store;
use crate::store::list_mocs;

use eframe::egui;
use egui::DragValue;
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
    Opcrea(creation_type),
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
    creation: CreationUis,
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
                    Op::Opcrea(creation_type::Cone),
                    "MOC creation",
                );
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
                Op::One(o) => self.moc_op1(ui, o.clone()),
                Op::Two(t) => self.moc_op2(ui, t.clone()),
                Op::Opnone => self.list_ui(ui),
                Op::Opcrea(c) => self.creation_ui(ui, c.clone()),
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
            creation: CreationUis::default(),
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
            ui.horizontal(|ui| {
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
                ui.separator();
                ui.label(self.error.as_ref().unwrap());
            }
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
                    if self.picked_file.is_some() {
                        if store::get_qty(&self.picked_file.clone().unwrap()) == Ok(Qty::Space) {
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
                    }
                });
        });
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
            if self.files_have_same_type() {
                if operation.eq(&Op2::SFold) || operation.eq(&Op2::TFold){
                    self.operation = Op::Two(Op2::Intersection);
                }
                let sel_text = format!("{}", operation);

                ui.horizontal(|ui| {
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
            } else if self.files_have_stmoc() {
                ui.horizontal(|ui| {
                    if store::get_qty(&self.picked_file.clone().unwrap()) == Ok(Qty::Space) 
                    || store::get_qty(&self.picked_second_file.clone().unwrap()) == Ok(Qty::Space)
                    {
                        ui.label("The only available operation is SFold, as such this operation as been set.");
                        self.operation = Op::Two(Op2::SFold);
                    } else if store::get_qty(&self.picked_file.clone().unwrap()) == Ok(Qty::Time) 
                    || store::get_qty(&self.picked_second_file.clone().unwrap()) == Ok(Qty::Time)
                    {
                        ui.label("The only available operation is TFold, as such this operation as been set.");
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

            //Combo box containing the different files that can be picked from
            ui.horizontal(|ui| {
                ui.label("MOC : ");
                self.make_cbox(ui, sel_text.as_str(), "file_cbox", None);
            });

            self.op_one_ui(ui, op.clone());

            //In case of degrade option ask for new depth
            let deg = matches!(op, Op1::Degrade { new_depth: _ });
            if deg {
                ui.add(egui::Slider::new(&mut self.deg, 0..=25));
            }

            if self.picked_file.is_some() {
                ui.horizontal(|ui| {
                    ui.label("New MOC name :");
                    ui.text_edit_singleline(&mut self.name);
                });

                //Button launching the operation
                if store::get_qty(&self.picked_file.clone().unwrap()) != Ok(Qty::Timespace) {
                    ui.horizontal(|ui| {
                        if ui.button("Launch").clicked() {
                            self.error = None;
                            if deg {
                                op = Op1::Degrade {
                                    new_depth: self.deg,
                                }
                            }
                            let moc = self.picked_file.clone().unwrap();

                            if self.name.len() == 0 {
                                if !op1(&moc, op, &format!("{}_{}", op.to_string(), moc)).is_ok() {
                                    self.error = Some("Error when trying to do operation".to_string());
                                }
                            } else {
                                if !op1(&moc, op, &self.name).is_ok() {
                                    self.error = Some("Error when trying to do operation".to_string());
                                    self.name = String::default();
                                }
                            }
                        };
                    });
                } else {
                    ui.label("SpaceTime MOCs cannot be operated on alone.");
                }
            }
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

            self.op_two_ui(ui, op.clone());

            if (self.picked_file.is_some() && self.picked_second_file.is_some()) && (self.files_have_same_type() || self.files_have_stmoc()) {
                ui.horizontal(|ui| {
                    ui.label("New MOC name :");
                    ui.text_edit_singleline(&mut self.name);
                });

                //Button launching the operation
                ui.horizontal(|ui| {
                    if ui.button("Launch").clicked() {
                        self.error = None;
                        let mut l = self.picked_file.as_ref().unwrap();
                        let mut r = self.picked_second_file.as_ref().unwrap();
                        if store::get_qty(l) == Ok(Qty::Timespace) {
                            let tmp = r;
                            r = l;
                            l = tmp;
                        }
                        if self.name.len() == 0 {
                            if !op2(&l, &r, op, &format!("{}_{}_{}", op.to_string(), l, r)).is_ok()
                            {
                                self.error = Some("Error when trying to do operation".to_string());
                            }
                        } else {
                            if !op2(&l, &r, op, &self.name).is_ok() {
                                self.error = Some("Error when trying to do operation".to_string());
                                self.name = String::default();
                            }
                        }
                    };
                });
            }
        }
    }

    fn creation_ui(&mut self, ui: &mut Ui, crea: creation_type) {
        let sel_text = format!("{}", crea);

        ui.horizontal(|ui| {
            ui.label("Creation type :");
            egui::ComboBox::from_id_source("Creation_cbox")
                .selected_text(sel_text)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.operation,
                        Op::Opcrea(creation_type::Cone),
                        "Cone",
                    );
                    ui.selectable_value(
                        &mut self.operation,
                        Op::Opcrea(creation_type::Ring),
                        "Ring",
                    );
                    ui.selectable_value(
                        &mut self.operation,
                        Op::Opcrea(creation_type::Elliptical_cone),
                        "Eliptical cone",
                    );
                    ui.selectable_value(
                        &mut self.operation,
                        Op::Opcrea(creation_type::Zone),
                        "Zone",
                    );
                    ui.selectable_value(
                        &mut self.operation,
                        Op::Opcrea(creation_type::Box),
                        "Box",
                    );
                });
        });

        let res = match crea {
            creation_type::Cone => self.creation.cone_ui(ui),
            creation_type::Ring => self.creation.ring_ui(ui),
            creation_type::Elliptical_cone => self.creation.eliptical_ui(ui),
            creation_type::Zone => self.creation.zone_ui(ui),
            creation_type::Box => self.creation.box_ui(ui),
            _ => todo!(),
        };
        if res.is_err() {
            res.unwrap_or_else(|e| self.error=Some(e));
        }else {
            self.error = None;
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
                                        self.error = Some("Error when trying to create file".to_string());
                                    }
                                }
                                if ui.button("ASCII").clicked() {
                                    if !to_ascii_file(filenames.get(row_index).unwrap(), Some(0))
                                        .is_ok()
                                    {
                                        self.error = Some("Error when trying to create file".to_string());
                                    }
                                }
                                if ui.button("JSON").clicked() {
                                    if !to_json_file(filenames.get(row_index).unwrap(), Some(0))
                                        .is_ok()
                                    {
                                        self.error = Some("Error when trying to create file".to_string());
                                    }
                                }
                            });
                        });
                        row.col(|ui| {
                            if ui.button("❌").clicked() {
                                if !store::drop(filenames.get(row_index).unwrap()).is_ok() {
                                    self.error = Some("Error when trying to remove file".to_string());
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

    fn files_have_stmoc(&mut self) -> bool {
        store::get_qty(&self.picked_second_file.clone().unwrap()) == Ok(Qty::Timespace)
                || store::get_qty(&self.picked_file.clone().unwrap()) == Ok(Qty::Timespace)
    }
    fn files_have_same_type(&mut self) -> bool {
        store::get_qty(&self.picked_file.clone().unwrap())
                .eq(&store::get_qty(&self.picked_second_file.clone().unwrap()))
    }
}

#[derive(Default)]
pub struct CreationUis {
    name: String,
    depth: u8,
    lon_deg: f64,
    lat_deg: f64,
    lon_deg_min: f64,
    lat_deg_min: f64,
    radius: f64,
    int_radius: f64,
    a_deg: f64,
    b_deg: f64,
    pa_deg: f64,
}
impl CreationUis {
    // UIs for types
    pub fn cone_ui(&mut self, ui: &mut Ui) -> Result<(), String> {
        self.depth_builder(ui);
        self.lon_lat_deg_builder(ui);
        self.radius_builder(ui);

        ui.horizontal(|ui| {
            ui.label("New MOC name :");
            ui.text_edit_singleline(&mut self.name);
        });

        if ui.button("create").clicked() {
            if self.name.len() == 0 {
                from_cone(&format!("Cone_of_rad_{}", self.radius.to_string().as_str()), self.depth, self.lon_deg, self.lat_deg, self.radius)?;
            } else {
                from_cone(&self.name, self.depth, self.lon_deg, self.lat_deg, self.radius)?;
            }
        }
        Ok(())
    }
    pub fn ring_ui(&mut self, ui: &mut Ui) -> Result<(), String> {
        self.depth_builder(ui);
        self.lon_lat_deg_builder(ui);
        self.radii_builder(ui);

        ui.horizontal(|ui| {
            ui.label("New MOC name :");
            ui.text_edit_singleline(&mut self.name);
        });

        if ui.button("create").clicked() {
            if self.name.len() == 0 {
                from_ring(&format!("ring_of_rad_{}_{}", self.int_radius.to_string().as_str(),self.radius.to_string().as_str()), 
                self.depth, self.lon_deg, self.lat_deg, self.int_radius,self.radius)?;
            } else {
                from_ring(&self.name, self.depth, self.lon_deg, self.lat_deg, self.int_radius, self.radius)?;
            }
        }
        Ok(())
    }
    pub fn eliptical_ui(&mut self, ui: &mut Ui) -> Result<(), String> {
        self.elipbox_builder(ui);

        if ui.button("create").clicked() {
            if self.name.len() == 0 {
                from_elliptical_cone(&format!("ElipCone_deg_{}_{}_{}", self.a_deg, self.b_deg, self.pa_deg), 
                self.depth, self.lon_deg, self.lat_deg, self.a_deg, self.b_deg, self.pa_deg)?;
            } else {
                from_elliptical_cone(&self.name, self.depth, self.lon_deg, self.lat_deg, self.a_deg, self.b_deg, self.pa_deg)?;
            }
        }
        Ok(())
    }
    pub fn zone_ui(&mut self, ui: &mut Ui) -> Result<(), String> {
        self.depth_builder(ui);
        self.lons_lats_builder(ui);

        if ui.button("create").clicked() {
            if self.name.len() == 0 {
                from_zone(&format!("Zone_deg_{}_{}", self.lon_deg_min, self.lat_deg_min), 
                self.depth, self.lon_deg_min, self.lat_deg_min, self.lon_deg, self.lat_deg)?;
            } else {
                from_zone(&self.name, self.depth, self.lon_deg_min, self.lat_deg_min, self.lon_deg, self.lat_deg)?;
            }
        }
        Ok(())
    }
    pub fn box_ui(&mut self, ui: &mut Ui) -> Result<(), String> {
        self.elipbox_builder(ui);

        if ui.button("create").clicked() {
            if self.name.len() == 0 {
                from_box(&format!("Box_deg_{}_{}_{}", self.a_deg, self.b_deg, self.pa_deg), 
                self.depth, self.lon_deg, self.lat_deg, self.a_deg, self.b_deg, self.pa_deg)?;
            } else {
                from_box(&self.name, self.depth, self.lon_deg, self.lat_deg, self.a_deg, self.b_deg, self.pa_deg)?;
            }
        }
        Ok(())
    }

    // COMMON BUILDERS
    fn elipbox_builder(&mut self, ui: &mut Ui) {
        self.depth_builder(ui);
        self.lon_lat_deg_builder(ui);
        self.degs_builder(ui);

        ui.horizontal(|ui| {
            ui.label("New MOC name :");
            ui.text_edit_singleline(&mut self.name);
        });
    }

    // BASE BUILDERS
    fn depth_builder(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Depth:");
            ui.add(egui::Slider::new(&mut self.depth, 0..=26));
        });
    }
    fn lon_lat_deg_builder(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Longitude degradation:");
            ui.add(egui::Slider::new(&mut self.lon_deg, 0.0..=TWICE_PI));
        });
        ui.horizontal(|ui| {
            ui.label("Latitude degradation:");
            ui.add(egui::Slider::new(&mut self.lat_deg, -HALF_PI..=HALF_PI));
        });
    }
    fn lons_lats_builder(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Minimal longitude degradation:");
            ui.add(egui::Slider::new(&mut self.lon_deg_min, 0.0..=self.lon_deg));
        });
        ui.horizontal(|ui| {
            ui.label("Minimal latitude degradation:");
            ui.add(egui::Slider::new(&mut self.lat_deg_min, -HALF_PI..=self.lat_deg));
        });
        ui.horizontal(|ui| {
            ui.label("Maximal longitude degradation:");
            ui.add(egui::Slider::new(&mut self.lon_deg, self.lon_deg_min..=TWICE_PI));
        });
        ui.horizontal(|ui| {
            ui.label("Maximal latitude degradation:");
            ui.add(egui::Slider::new(&mut self.lat_deg, self.lat_deg_min..=HALF_PI));
        });
    }
    fn radius_builder(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Radius:");
            ui.add(egui::Slider::new(&mut self.radius, 0.0..=PI));
        });
    }
    fn radii_builder(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Internal radius:");
            ui.add(egui::Slider::new(&mut self.int_radius, 0.0..=self.radius));
        });
        ui.horizontal(|ui| {
            ui.label("External radius:");
            ui.add(egui::Slider::new(&mut self.radius, self.int_radius..=PI));
        });
    }
    fn degs_builder(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui|{
            ui.label("A degradation:");
            ui.add(egui::Slider::new(&mut self.a_deg, 0.0..=HALF_PI));
        });
        ui.horizontal(|ui|{
            ui.label("B degradation:");
            ui.add(egui::Slider::new(&mut self.b_deg, 0.0..=self.a_deg));
        });
        ui.horizontal(|ui|{
            ui.label("PA degradation:");
            ui.add(egui::Slider::new(&mut self.pa_deg, 0.0..=PI));
        });
    }
}
