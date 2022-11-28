#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::op::creation::*;
use crate::{app::log, commons::*};

use super::creationui::CreationType;
use eframe::egui;
use egui::Ui;
use rfd::AsyncFileDialog;

#[derive(Default)]
pub struct CreationUis {
    name: String,
    depth: u8,
    lon_deg_polf1: f64,
    lat_deg_polf2: f64,
    radius_a: f64,
    lon_deg_min_b_int: f64,
    lat_deg_min_pa: f64,
    comp: bool,
    typ: CreationType,
    error: Option<String>,
}
impl CreationUis {
    pub(crate) fn creation_ui(&mut self, ui: &mut Ui) {
        let sel_text = format!("{}", self.typ);

        ui.horizontal(|ui| {
            ui.label("Creation type :");
            egui::ComboBox::from_id_source("Creation_cbox")
                .selected_text(sel_text)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.typ, CreationType::Cone, "Cone");
                    ui.selectable_value(&mut self.typ, CreationType::Ring, "Ring");
                    ui.selectable_value(
                        &mut self.typ,
                        CreationType::EllipticalCone,
                        "Eliptical cone",
                    );
                    ui.selectable_value(&mut self.typ, CreationType::Zone, "Zone");
                    ui.selectable_value(&mut self.typ, CreationType::Box, "Box");
                    ui.selectable_value(&mut self.typ, CreationType::Polygon, "Polygon");
                    ui.selectable_value(&mut self.typ, CreationType::Coo, "Coo");
                    ui.selectable_value(&mut self.typ, CreationType::SmallCone, "Cone S");
                    ui.selectable_value(&mut self.typ, CreationType::LargeCone, "Cone L");
                    ui.selectable_value(&mut self.typ, CreationType::DecimalJd, "Time: dec");
                    ui.selectable_value(&mut self.typ, CreationType::DecimalJdRange, "Time: range");
                });
        });

        match self.typ {
            CreationType::Cone => self.error = self.cone_ui(ui, &self.error.clone()),
            CreationType::Ring => self.error = self.ring_ui(ui, &self.error.clone()),
            CreationType::EllipticalCone => self.error = self.eliptical_ui(ui, &self.error.clone()),
            CreationType::Zone => self.error = self.zone_ui(ui, &self.error.clone()),
            CreationType::Box => self.error = self.box_ui(ui, &self.error.clone()),
            CreationType::Polygon => self.error = self.polygon_ui(ui, &self.error.clone()),
            CreationType::Coo => self.error = self.coo_ui(ui, &self.error.clone()),
            CreationType::SmallCone => self.error = self.smallc_ui(ui, &self.error.clone()),
            CreationType::LargeCone => self.error = self.largec_ui(ui, &self.error.clone()),
            CreationType::DecimalJd => self.error = self.jd_ui(ui, &self.error.clone()),
            CreationType::DecimalJdRange => self.error = self.jdr_ui(ui, &self.error.clone()),
            _ => todo!(),
        };
    }

    // UIs for types
    pub(crate) fn cone_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();
        self.depth_builder(ui);
        self.lon_lat_deg_builder(ui);
        self.radius_builder(ui);

        ui.horizontal(|ui| {
            ui.label("New MOC name :");
            ui.text_edit_singleline(&mut self.name);
        });

        if ui.button("Create").clicked() {
            err = None;
            if self.name.is_empty() {
                if let Err(e) = from_cone(
                    &format!("Cone_of_rad_{}", self.radius_a.to_string().as_str()),
                    self.depth,
                    self.lon_deg_polf1,
                    self.lat_deg_polf2,
                    self.radius_a,
                ) {
                    err = Some(e);
                }
            } else if let Err(e) = from_cone(
                &self.name,
                self.depth,
                self.lon_deg_polf1,
                self.lat_deg_polf2,
                self.radius_a,
            ) {
                err = Some(e);
            }
        }
        err
    }
    pub(crate) fn ring_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();
        self.depth_builder(ui);
        self.lon_lat_deg_builder(ui);
        self.radii_builder(ui);

        ui.horizontal(|ui| {
            ui.label("New MOC name :");
            ui.text_edit_singleline(&mut self.name);
        });

        if ui.button("Create").clicked() {
            err = None;
            if self.name.is_empty() {
                if let Err(e) = from_ring(
                    &format!(
                        "ring_of_rad_{}_{}",
                        self.lon_deg_min_b_int.to_string().as_str(),
                        self.radius_a.to_string().as_str()
                    ),
                    self.depth,
                    self.lon_deg_polf1,
                    self.lat_deg_polf2,
                    self.lon_deg_min_b_int,
                    self.radius_a,
                ) {
                    err = Some(e);
                }
            } else if let Err(e) = from_ring(
                &self.name,
                self.depth,
                self.lon_deg_polf1,
                self.lat_deg_polf2,
                self.lon_deg_min_b_int,
                self.radius_a,
            ) {
                err = Some(e);
            }
        }
        err
    }
    pub(crate) fn eliptical_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();
        self.elipbox_builder(ui);

        if ui.button("Create").clicked() {
            err = None;
            if self.name.is_empty() {
                if let Err(e) = from_elliptical_cone(
                    &format!(
                        "ElipCone_deg_{}_{}_{}",
                        self.radius_a, self.lon_deg_min_b_int, self.lat_deg_min_pa,
                    ),
                    self.depth,
                    self.lon_deg_polf1,
                    self.lat_deg_polf2,
                    self.radius_a,
                    self.lon_deg_min_b_int,
                    self.lat_deg_min_pa,
                ) {
                    err = Some(e);
                }
            } else if let Err(e) = from_elliptical_cone(
                &self.name,
                self.depth,
                self.lon_deg_polf1,
                self.lat_deg_polf2,
                self.radius_a,
                self.lon_deg_min_b_int,
                self.lat_deg_min_pa,
            ) {
                err = Some(e);
            }
        }
        err
    }
    pub(crate) fn zone_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();
        self.depth_builder(ui);
        self.lons_lats_builder(ui);

        if ui.button("Create").clicked() {
            err = None;
            if self.name.is_empty() {
                if let Err(e) = from_zone(
                    &format!(
                        "Zone_deg_{}_{}",
                        self.lon_deg_min_b_int, self.lat_deg_min_pa
                    ),
                    self.depth,
                    self.lon_deg_min_b_int,
                    self.lat_deg_min_pa,
                    self.lon_deg_polf1,
                    self.lat_deg_polf2,
                ) {
                    err = Some(e);
                }
            } else if let Err(e) = from_zone(
                &self.name,
                self.depth,
                self.lon_deg_min_b_int,
                self.lat_deg_min_pa,
                self.lon_deg_polf1,
                self.lat_deg_polf2,
            ) {
                err = Some(e);
            }
        }
        err
    }

    pub(crate) fn box_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();
        self.elipbox_builder(ui);

        if ui.button("Create").clicked() {
            err = None;
            if self.name.is_empty() {
                if let Err(e) = from_box(
                    &format!(
                        "Box_deg_{}_{}_{}",
                        self.radius_a, self.lon_deg_min_b_int, self.lat_deg_min_pa
                    ),
                    self.depth,
                    self.lon_deg_polf1,
                    self.lat_deg_polf2,
                    self.radius_a,
                    self.lon_deg_min_b_int,
                    self.lat_deg_min_pa,
                ) {
                    err = Some(e);
                }
            } else if let Err(e) = from_box(
                &self.name,
                self.depth,
                self.lon_deg_polf1,
                self.lat_deg_polf2,
                self.radius_a,
                self.lon_deg_min_b_int,
                self.lat_deg_min_pa,
            ) {
                err = Some(e);
            }
        }
        err
    }

    pub(crate) fn polygon_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();

        self.depth_builder(ui);
        self.check_bool(ui, "complement");

        ui.horizontal(|ui| {
            ui.label("New MOC name :");
            ui.text_edit_singleline(&mut self.name);
        });

        if ui.button("Create").clicked() {
            err = None;

            if let Err(e) = self.load_csv(CreationType::Coo) {
                err = Some(e);
            }
        }
        err
    }

    pub(crate) fn coo_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        self.coo_cones_jd_builder(ui, CreationType::Coo, e)
    }
    pub(crate) fn smallc_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        self.coo_cones_jd_builder(ui, CreationType::SmallCone, e)
    }
    pub(crate) fn largec_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        self.coo_cones_jd_builder(ui, CreationType::LargeCone, e)
    }
    pub(crate) fn jd_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        self.coo_cones_jd_builder(ui, CreationType::DecimalJd, e)
    }
    pub(crate) fn jdr_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        self.coo_cones_jd_builder(ui, CreationType::DecimalJdRange, e)
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

    fn coo_cones_jd_builder(
        &mut self,
        ui: &mut Ui,
        typ: CreationType,
        e: &Option<String>,
    ) -> Option<String> {
        let mut err = e.to_owned();

        self.depth_builder(ui);

        ui.horizontal(|ui| {
            ui.label("New MOC name :");
            ui.text_edit_singleline(&mut self.name);
        });

        ui.label("Creating a MOC like this will ask you for a .csv file.");

        if ui.button("Create").clicked() {
            err = None;
            if let Err(e) = self.load_csv(typ) {
                err = Some(e);
            }
        }
        err
    }

    // BASE BUILDERS
    fn depth_builder(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Depth:");
            ui.add(egui::Slider::new(&mut self.depth, 0..=26));
        });
    }
    fn lon_lat_deg_builder(&mut self, ui: &mut Ui) {
        if !(0.0..=TWICE_PI).contains(&self.lon_deg_polf1)
            || !(-HALF_PI..=HALF_PI).contains(&self.lat_deg_polf2)
        {
            self.lon_deg_polf1 = 0.0;
            self.lat_deg_polf2 = 0.0;
        }

        ui.horizontal(|ui| {
            ui.label("Longitude degradation:");
            ui.add(egui::Slider::new(&mut self.lon_deg_polf1, 0.0..=TWICE_PI));
        });
        ui.horizontal(|ui| {
            ui.label("Latitude degradation:");
            ui.add(egui::Slider::new(
                &mut self.lat_deg_polf2,
                -HALF_PI..=HALF_PI,
            ));
        });
    }
    fn lons_lats_builder(&mut self, ui: &mut Ui) {
        if !(0.0..=self.lon_deg_polf1).contains(&self.lon_deg_min_b_int)
            || !(-HALF_PI..=self.lat_deg_polf2).contains(&self.lat_deg_min_pa)
        {
            self.lon_deg_min_b_int = 0.0;
            self.lat_deg_min_pa = 0.0;
        }

        ui.horizontal(|ui| {
            ui.label("Minimal longitude degradation:");
            ui.add(egui::Slider::new(
                &mut self.lon_deg_min_b_int,
                0.0..=self.lon_deg_polf1,
            ));
        });
        ui.horizontal(|ui| {
            ui.label("Minimal latitude degradation:");
            ui.add(egui::Slider::new(
                &mut self.lat_deg_min_pa,
                -HALF_PI..=self.lat_deg_polf2,
            ));
        });
        ui.horizontal(|ui| {
            ui.label("Maximal longitude degradation:");
            ui.add(egui::Slider::new(
                &mut self.lon_deg_polf1,
                self.lon_deg_min_b_int..=TWICE_PI,
            ));
        });
        ui.horizontal(|ui| {
            ui.label("Maximal latitude degradation:");
            ui.add(egui::Slider::new(
                &mut self.lat_deg_polf2,
                self.lat_deg_min_pa..=HALF_PI,
            ));
        });
    }
    fn radius_builder(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Radius:");
            ui.add(egui::Slider::new(&mut self.radius_a, 0.0..=PI));
        });
    }
    fn radii_builder(&mut self, ui: &mut Ui) {
        if !(self.lon_deg_min_b_int..=PI).contains(&self.radius_a)
            || !(0.0..=self.radius_a).contains(&self.lon_deg_min_b_int)
        {
            self.radius_a = 0.0;
            self.lon_deg_min_b_int = 0.0;
        }
        ui.horizontal(|ui| {
            ui.label("Internal radius:");
            ui.add(egui::Slider::new(
                &mut self.lon_deg_min_b_int,
                0.0..=self.radius_a,
            ));
        });
        ui.horizontal(|ui| {
            ui.label("External radius:");
            ui.add(egui::Slider::new(
                &mut self.radius_a,
                self.lon_deg_min_b_int..=PI,
            ));
        });
    }
    fn degs_builder(&mut self, ui: &mut Ui) {
        if !(0.0..=HALF_PI).contains(&self.radius_a)
            || !(0.0..=self.radius_a).contains(&self.lon_deg_min_b_int)
            || !(0.0..=PI).contains(&self.lat_deg_min_pa)
        {
            self.radius_a = 0.0;
            self.lon_deg_min_b_int = 0.0;
            self.lat_deg_min_pa = 0.0;
        }

        ui.horizontal(|ui| {
            ui.label("A degradation:");
            ui.add(egui::Slider::new(&mut self.radius_a, 0.0..=HALF_PI));
        });
        ui.horizontal(|ui| {
            ui.label("B degradation:");
            ui.add(egui::Slider::new(
                &mut self.lon_deg_min_b_int,
                0.0..=self.radius_a,
            ));
        });
        ui.horizontal(|ui| {
            ui.label("PA degradation:");
            ui.add(egui::Slider::new(&mut self.lat_deg_min_pa, 0.0..=PI));
        });
    }
    fn check_bool(&mut self, ui: &mut Ui, txt: &str) {
        ui.checkbox(&mut self.comp, txt);
    }

    fn load_csv(&mut self, typ: CreationType) -> Result<(), String> {
        let task = AsyncFileDialog::new()
            .add_filter("MOCs", &["csv"])
            .pick_file();

        let depth = self.depth;
        let mut name = format!("Coo_{}", self.depth);
        if !self.name.is_empty() {
            name = self.name.clone();
        }
        let complement = self.comp;

        execute(async move {
            let handle = task.await;
            if let Some(file) = handle {
                let file_content = unsafe { String::from_utf8_unchecked(file.read().await) };

                let _ = match typ {
                    CreationType::Box => todo!(),
                    CreationType::Cone => todo!(),
                    CreationType::Coo => from_coo(&name, depth, file_content),
                    CreationType::DecimalJd => from_decimal_jd(&name, depth, file_content),
                    CreationType::DecimalJdRange => {
                        from_decimal_jd_range(&name, depth, file_content)
                    }
                    CreationType::EllipticalCone => todo!(),
                    CreationType::LargeCone => from_large_cones(&name, depth, file_content),
                    CreationType::Polygon => from_polygon(&name, depth, file_content, complement),
                    CreationType::Ring => todo!(),
                    CreationType::SmallCone => from_small_cones(&name, depth, file_content),
                    CreationType::ValuedCells => todo!(),
                    CreationType::Zone => todo!(),
                };
            }
        });
        Ok(())
    }
}

fn execute<F: std::future::Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
