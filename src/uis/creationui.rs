#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::commons::*;
use crate::op::{creation::*};

use eframe::egui;
use egui::Ui;

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
    pub fn cone_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();
        self.depth_builder(ui);
        self.lon_lat_deg_builder(ui);
        self.radius_builder(ui);

        ui.horizontal(|ui| {
            ui.label("New MOC name :");
            ui.text_edit_singleline(&mut self.name);
        });

        if ui.button("create").clicked() {
            err = None;
            if self.name.is_empty() {
                if let Err(e) = from_cone(
                    &format!("Cone_of_rad_{}", self.radius.to_string().as_str()),
                    self.depth,
                    self.lon_deg,
                    self.lat_deg,
                    self.radius,
                ) {
                    err = Some(e);
                }
            } else if let Err(e) = from_cone(
                &self.name,
                self.depth,
                self.lon_deg,
                self.lat_deg,
                self.radius,
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

        if ui.button("create").clicked() {
            err = None;
            if self.name.is_empty() {
                if let Err(e) = from_ring(
                    &format!(
                        "ring_of_rad_{}_{}",
                        self.int_radius.to_string().as_str(),
                        self.radius.to_string().as_str()
                    ),
                    self.depth,
                    self.lon_deg,
                    self.lat_deg,
                    self.int_radius,
                    self.radius,
                ) {
                    err = Some(e);
                }
            } else if let Err(e) = from_ring(
                &self.name,
                self.depth,
                self.lon_deg,
                self.lat_deg,
                self.int_radius,
                self.radius,
            ) {
                err = Some(e);
            }
        }
        err
    }
    pub fn eliptical_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();
        self.elipbox_builder(ui);

        if ui.button("create").clicked() {
            err = None;
            if self.name.is_empty() {
                if let Err(e) = from_elliptical_cone(
                    &format!("ElipCone_deg_{}_{}_{}", self.a_deg, self.b_deg, self.pa_deg),
                    self.depth,
                    self.lon_deg,
                    self.lat_deg,
                    self.a_deg,
                    self.b_deg,
                    self.pa_deg,
                ) {
                    err = Some(e);
                }
            } else if let Err(e) = from_elliptical_cone(
                &self.name,
                self.depth,
                self.lon_deg,
                self.lat_deg,
                self.a_deg,
                self.b_deg,
                self.pa_deg,
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

        if ui.button("create").clicked() {
            err = None;
            if self.name.is_empty() {
                if let Err(e) = from_zone(
                    &format!("Zone_deg_{}_{}", self.lon_deg_min, self.lat_deg_min),
                    self.depth,
                    self.lon_deg_min,
                    self.lat_deg_min,
                    self.lon_deg,
                    self.lat_deg,
                ) {
                    err = Some(e);
                }
            } else if let Err(e) = from_zone(
                &self.name,
                self.depth,
                self.lon_deg_min,
                self.lat_deg_min,
                self.lon_deg,
                self.lat_deg,
            ) {
                err = Some(e);
            }
        }
        err
    }
    pub fn box_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();
        self.elipbox_builder(ui);

        if ui.button("create").clicked() {
            err = None;
            if self.name.is_empty() {
                if let Err(e) = from_box(
                    &format!("Box_deg_{}_{}_{}", self.a_deg, self.b_deg, self.pa_deg),
                    self.depth,
                    self.lon_deg,
                    self.lat_deg,
                    self.a_deg,
                    self.b_deg,
                    self.pa_deg,
                ) {
                    err = Some(e);
                }
            } else if let Err(e) = from_box(
                &self.name,
                self.depth,
                self.lon_deg,
                self.lat_deg,
                self.a_deg,
                self.b_deg,
                self.pa_deg,
            ) {
                err = Some(e);
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
            ui.add(egui::Slider::new(
                &mut self.lat_deg_min,
                -HALF_PI..=self.lat_deg,
            ));
        });
        ui.horizontal(|ui| {
            ui.label("Maximal longitude degradation:");
            ui.add(egui::Slider::new(
                &mut self.lon_deg,
                self.lon_deg_min..=TWICE_PI,
            ));
        });
        ui.horizontal(|ui| {
            ui.label("Maximal latitude degradation:");
            ui.add(egui::Slider::new(
                &mut self.lat_deg,
                self.lat_deg_min..=HALF_PI,
            ));
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
        ui.horizontal(|ui| {
            ui.label("A degradation:");
            ui.add(egui::Slider::new(&mut self.a_deg, 0.0..=HALF_PI));
        });
        ui.horizontal(|ui| {
            ui.label("B degradation:");
            ui.add(egui::Slider::new(&mut self.b_deg, 0.0..=self.a_deg));
        });
        ui.horizontal(|ui| {
            ui.label("PA degradation:");
            ui.add(egui::Slider::new(&mut self.pa_deg, 0.0..=PI));
        });
    }
}
