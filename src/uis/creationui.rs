#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::commons::*;
use crate::op::creation::*;

use eframe::egui;
use egui::{DragValue, Ui};
use egui_extras::{Size, TableBuilder};

#[derive(Default)]
pub struct CreationUis {
    name: String,
    depth: u8,
    lon_deg_polf1: f64,
    lat_deg_polf2: f64,
    radius_a: f64,
    lon_deg_min_b_int: f64,
    lat_deg_min_pa: f64,
    vert_coos: Vec<(f64, f64)>,
    coos_radius: Box<[f64]>,
    uniqs: Box<[f64]>,
    values: Box<[f64]>,
}
impl CreationUis {
    // UIs for types
    pub fn cone_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
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
    pub fn ring_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
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
    pub fn eliptical_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
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
    pub fn zone_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
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
    pub fn box_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
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

    pub fn polygon_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();

        let txt_h = 30.0;
        ui.vertical(|ui| {
            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Size::initial(150.0).at_least(100.0))
                .column(Size::initial(150.0).at_least(100.0))
                .column(Size::remainder().at_least(20.0))
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("deg1");
                        ui.add(DragValue::new(&mut self.lon_deg_polf1));
                    });
                    header.col(|ui| {
                        ui.heading("deg2");
                        ui.add(DragValue::new(&mut self.lat_deg_polf2));
                        if ui.button("add").clicked() {
                            if self.lon_deg_polf1.eq(&self.lat_deg_polf2) || self.vert_coos.contains(&(self.lon_deg_polf1, self.lat_deg_polf2)) {
                                err = Some("Please add 2 different values that are not already present in your polygon".to_string());
                            } else {
                                self.vert_coos.push((self.lon_deg_polf1, self.lat_deg_polf2));
                                self.lon_deg_polf1 = 0.0;
                                self.lat_deg_polf2 = 0.0;
                            }
                        }
                    });
                    header.col(|ui| {
                        ui.heading("❌");
                    });
                })
                .body(|body| {
                    body.rows(txt_h, self.vert_coos.len(), |row_index, mut row| {
                        row.col(|ui| {
                            ui.label(self.vert_coos.get(row_index).or(Some(&(0.0,0.0))).unwrap().0.to_string());
                        });
                        row.col(|ui| {
                            ui.label(self.vert_coos.get(row_index).or(Some(&(0.0,0.0))).unwrap().1.to_string());
                        });
                        row.col(|ui| {
                            if ui.button("❌").clicked() {
                                self.vert_coos.remove(row_index);
                            }
                        });
                    })
                })
        });

        if ui.button("Create").clicked() {
            err = None;
            if self.vert_coos.len() < 3 {
                err = Some(
                    "You need to add at least 3 vertices coordinates to make a polygon".to_string(),
                );
            }
        }
        err
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
}
