#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::controllers::creation::*;

use super::creationui::CreationType;
use eframe::egui;
use egui::Ui;

#[cfg(target_arch = "wasm32")]
use rfd::AsyncFileDialog;

#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;
use std::fs::File;
use std::io::Read;

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
    // Valued cells requirements
    density: bool,
    asc: bool,
    not_strict: bool,
    split: bool,
    revese_recursive_descent: bool,
    from_threshold: f64,
    to_threshold: f64,
}
impl CreationUis {
    ////////////////////////////////////////////////
    // MAIN UI (this uses the sub UIs seen later) //

    // #Definition
    //      Creation_ui, the main UI component for MOC creation
    // #Args
    //  *   `ui`: the egui UI that needs to show the given components
    pub(crate) fn creation_ui(&mut self, ui: &mut Ui) -> Result<(), String> {
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
                    ui.selectable_value(&mut self.typ, CreationType::ValuedCells, "Valued cells");
                });
        });

        // The small paragraph before the match sets a grid layout to have every element aligned
        egui::Grid::new("my_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                match self.typ {
                    CreationType::Cone => self.error = self.cone_ui(ui, &self.error.clone()),
                    CreationType::Ring => self.error = self.ring_ui(ui, &self.error.clone()),
                    CreationType::EllipticalCone => {
                        self.error = self.eliptical_ui(ui, &self.error.clone())
                    }
                    CreationType::Zone => self.error = self.zone_ui(ui, &self.error.clone()),
                    CreationType::Box => self.error = self.box_ui(ui, &self.error.clone()),
                    CreationType::Polygon => self.error = self.polygon_ui(ui, &self.error.clone()),
                    CreationType::Coo => self.error = self.coo_ui(ui, &self.error.clone()),
                    CreationType::SmallCone => self.error = self.smallc_ui(ui, &self.error.clone()),
                    CreationType::LargeCone => self.error = self.largec_ui(ui, &self.error.clone()),
                    CreationType::DecimalJd => self.error = self.jd_ui(ui, &self.error.clone()),
                    CreationType::DecimalJdRange => {
                        self.error = self.jdr_ui(ui, &self.error.clone())
                    }
                    CreationType::ValuedCells => {
                        self.error = self.valued_c(ui, &self.error.clone())
                    }
                };
            });
        if self.error.is_some() {
            return Err(self.error.clone().unwrap());
        }
        Ok(())
    }

    /////////////
    // sub-UIs //

    //////////////////////////////////////////////////////////////////////////
    // All sub-Uis are defined the same and follow a "Type" of MOC creation.
    // #Definition
    //      [Type]_ui, the UI for creation from a [Type]
    // #Args
    //  *   `ui`: the egui UI that needs to show the given components
    //  *   `e`: an optional String in case of past errors to keep it visible until change
    // #Errors
    //      Depending on the outcome of the "from_[Type]" Moc creation operation
    //      the program may return an error
    //////////////////////////////////////////////////////////////////////////

    pub(crate) fn cone_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();
        self.depth_builder(ui);
        self.lon_lat_deg_builder(ui);
        self.radius_builder(ui);

        ui.label("New MOC name :");
        ui.text_edit_singleline(&mut self.name);
        ui.end_row();

        if ui.button("Create").clicked() {
            err = None;
            if self.name.is_empty() {
                self.name = format!("Cone_of_rad_{}", self.radius_a.to_string().as_str());
            }
            let _ = from_cone(
                &self.name,
                self.depth,
                self.lon_deg_polf1,
                self.lat_deg_polf2,
                self.radius_a,
            )
            .map_err(|e| err = Some(e));
            self.name = String::default();
        }
        err
    }
    pub(crate) fn ring_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();
        self.depth_builder(ui);
        self.lon_lat_deg_builder(ui);
        self.radii_builder(ui);

        ui.label("New MOC name :");
        ui.text_edit_singleline(&mut self.name);
        ui.end_row();

        if ui.button("Create").clicked() {
            err = None;
            if self.name.is_empty() {
                self.name = format!(
                    "ring_of_rad_{}_{}",
                    self.lon_deg_min_b_int.to_string().as_str(),
                    self.radius_a.to_string().as_str()
                );
            }
            let _ = from_ring(
                &self.name,
                self.depth,
                self.lon_deg_polf1,
                self.lat_deg_polf2,
                self.lon_deg_min_b_int,
                self.radius_a,
            )
            .map_err(|e| err = Some(e));
            self.name = String::default();
        }
        err
    }
    pub(crate) fn eliptical_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();
        self.elipbox_builder(ui);

        if ui.button("Create").clicked() {
            err = None;
            if self.name.is_empty() {
                self.name = format!(
                    "ElipCone_deg_{}_{}_{}",
                    self.radius_a, self.lon_deg_min_b_int, self.lat_deg_min_pa,
                )
            }
            let _ = from_elliptical_cone(
                &self.name,
                self.depth,
                self.lon_deg_polf1,
                self.lat_deg_polf2,
                self.radius_a,
                self.lon_deg_min_b_int,
                self.lat_deg_min_pa,
            )
            .map_err(|e| err = Some(e));
            self.name = String::default();
        }
        err
    }
    pub(crate) fn zone_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();
        self.depth_builder(ui);
        self.lons_lats_builder(ui);

        ui.label("New MOC name :");
        ui.text_edit_singleline(&mut self.name);
        ui.end_row();

        if ui.button("Create").clicked() {
            err = None;
            if self.name.is_empty() {
                self.name = format!(
                    "Zone_deg_{}_{}",
                    self.lon_deg_min_b_int, self.lat_deg_min_pa
                );
            }
            let _ = from_zone(
                &self.name,
                self.depth,
                self.lon_deg_min_b_int,
                self.lat_deg_min_pa,
                self.lon_deg_polf1,
                self.lat_deg_polf2,
            )
            .map_err(|e| err = Some(e));
            self.name = String::default();
        }
        err
    }

    pub(crate) fn box_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();
        self.elipbox_builder(ui);

        if ui.button("Create").clicked() {
            err = None;
            if self.name.is_empty() {
                self.name = format!(
                    "Box_deg_{}_{}_{}",
                    self.radius_a, self.lon_deg_min_b_int, self.lat_deg_min_pa
                );
            }
            let _ = from_box(
                &self.name,
                self.depth,
                self.lon_deg_polf1,
                self.lat_deg_polf2,
                self.radius_a,
                self.lon_deg_min_b_int,
                self.lat_deg_min_pa,
            )
            .map_err(|e| err = Some(e));
            self.name = String::default();
        }
        err
    }

    pub(crate) fn polygon_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();

        self.depth_builder(ui);
        ui.checkbox(&mut self.comp, "Complement");
        ui.end_row();

        ui.label("New MOC name :");
        ui.text_edit_singleline(&mut self.name);
        ui.end_row();

        ui.label("Creating a MOC like this will ask you for a .csv file.");
        ui.end_row();

        if ui.button("Open file & create").clicked() {
            err = None;

            let _ = self.load_csv(CreationType::Coo).map_err(|e| err = Some(e));
            self.name = String::default();
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

    // Jd_ui and Jdr_ui are different, they permit Time MOCs creation.
    pub(crate) fn jd_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        self.coo_cones_jd_builder(ui, CreationType::DecimalJd, e)
    }
    pub(crate) fn jdr_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        self.coo_cones_jd_builder(ui, CreationType::DecimalJdRange, e)
    }

    #[cfg(target_arch = "wasm32")]
    fn valued_c(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.clone();

        self.depth_builder(ui);
        self.threshold_builder(ui);

        ui.checkbox(&mut self.density, "Density");
        ui.end_row();
        ui.checkbox(&mut self.asc, "Asc");
        ui.end_row();
        ui.checkbox(&mut self.not_strict, "Strict");
        ui.end_row();
        ui.checkbox(&mut self.split, "Split");
        ui.end_row();
        ui.checkbox(
            &mut self.revese_recursive_descent,
            "Revese recursive descent",
        );
        ui.end_row();
        ui.label("New MOC name :");
        ui.text_edit_singleline(&mut self.name);
        ui.end_row();

        if ui.button("Open file & create").clicked() {
            err = None;

            let task = AsyncFileDialog::new()
                .add_filter("MOCs", &["csv"])
                .pick_file();

            let depth = self.depth;
            let mut name = format!("ValuedC_{}", self.depth);
            if !self.name.is_empty() {
                name = self.name.clone();
            }
            let density = self.density;
            let asc = self.asc;
            let not_strict = self.not_strict;
            let split = self.split;
            let revese_recursive_descent = self.revese_recursive_descent;
            let from_threshold = self.from_threshold;
            let to_threshold = self.to_threshold;

            execute(async move {
                let handle = task.await;
                if let Some(file) = handle {
                    let file_content = unsafe { String::from_utf8_unchecked(file.read().await) };
                    let _ = from_valued_cells(
                        &name,
                        depth,
                        density,
                        from_threshold,
                        to_threshold,
                        asc,
                        not_strict,
                        split,
                        revese_recursive_descent,
                        file_content,
                    );
                }
            });
        }
        err
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn valued_c(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.clone();

        self.depth_builder(ui);
        self.threshold_builder(ui);

        ui.checkbox(&mut self.density, "Density");
        ui.end_row();
        ui.checkbox(&mut self.asc, "Asc");
        ui.end_row();
        ui.checkbox(&mut self.not_strict, "Strict");
        ui.end_row();
        ui.checkbox(&mut self.split, "Split");
        ui.end_row();
        ui.checkbox(
            &mut self.revese_recursive_descent,
            "Revese recursive descent",
        );
        ui.end_row();
        ui.label("New MOC name :");
        ui.text_edit_singleline(&mut self.name);
        ui.end_row();

        if ui.button("Open file & create").clicked() {
            err = None;

            if let Some(path) = FileDialog::new().add_filter("MOCs", &["csv"]).pick_file() {
                let depth = self.depth;
                let mut name = format!("ValuedC_{}", self.depth);
                if !self.name.is_empty() {
                    name = self.name.clone();
                }

                let mut file = File::open(&path)
                    .map_err(|_| err = Some("Error while opening file".to_string()))
                    .unwrap();
                let mut file_content = Vec::default();
                file.read_to_end(&mut file_content)
                    .map_err(|e| err = Some(e.to_string()))
                    .unwrap();
                let file_content = unsafe { String::from_utf8_unchecked(file_content) };

                let density = self.density;
                let asc = self.asc;
                let not_strict = self.not_strict;
                let split = self.split;
                let revese_recursive_descent = self.revese_recursive_descent;
                let from_threshold = self.from_threshold;
                let to_threshold = self.to_threshold;

                let _ = from_valued_cells(
                    &name,
                    depth,
                    density,
                    from_threshold,
                    to_threshold,
                    asc,
                    not_strict,
                    split,
                    revese_recursive_descent,
                    file_content,
                )
                .map_err(|e| err = Some(e));
            }
        }
        err
    }

    /////////////////////
    // COMMON BUILDERS //

    // #Definition
    //      Elipbox_builder is a function that helps with the creation of both
    //      the eliptical cone UI and the box UI.
    // #Args
    //  *   `ui`: the egui UI that needs to show the given components
    fn elipbox_builder(&mut self, ui: &mut Ui) {
        self.depth_builder(ui);
        self.lon_lat_deg_builder(ui);
        self.degs_builder(ui);

        ui.label("New MOC name :");
        ui.text_edit_singleline(&mut self.name);
        ui.end_row();
    }

    // #Definition
    //      Coo_cones_jd_builder is a function that helps with the creation of
    //      the coo UI, the small and large cone UI and both decimal jd UIs.
    // #Args
    //  *   `ui`: the egui UI that needs to show the given components
    //  *   `typ`: the type of creation we are currently performing
    //  *   `e`: an optional String in case of past errors to keep it visible until change
    // #Errors
    //      Depending on the outcome of the "from_[Type]" Moc creation operation
    //      the program may return an error
    fn coo_cones_jd_builder(
        &mut self,
        ui: &mut Ui,
        typ: CreationType,
        e: &Option<String>,
    ) -> Option<String> {
        let mut err = e.to_owned();

        self.depth_builder(ui);

        ui.label("New MOC name :");
        ui.text_edit_singleline(&mut self.name);
        ui.end_row();

        ui.label("Creating a MOC like this will ask you for a .csv file.");
        ui.end_row();

        if ui.button("Open file & create").clicked() {
            err = None;
            let _ = self.load_csv(typ).map_err(|e| err = Some(e));
            self.name = String::default();
        }
        err
    }

    ////////////////////
    // BASIC BUILDERS //

    fn depth_builder(&mut self, ui: &mut Ui) {
        ui.label("Depth:");
        ui.add(egui::Slider::new(&mut self.depth, 0..=26));
        ui.end_row();
    }

    fn lon_lat_deg_builder(&mut self, ui: &mut Ui) {
        ui.label("Longitude degradation:");
        ui.add(egui::Slider::new(&mut self.lon_deg_polf1, 0.0..=360.0).suffix("°"));
        ui.end_row();
        ui.label("Latitude degradation:");
        ui.add(egui::Slider::new(&mut self.lat_deg_polf2, -90.0..=90.0).suffix("°"));
        ui.end_row();
    }

    fn lons_lats_builder(&mut self, ui: &mut Ui) {
        ui.label("Minimal longitude degradation:");
        ui.add(
            egui::Slider::new(&mut self.lon_deg_min_b_int, 0.0..=self.lon_deg_polf1).suffix("°"),
        );
        ui.end_row();
        ui.label("Minimal latitude degradation:");
        ui.add(egui::Slider::new(&mut self.lat_deg_min_pa, -90.0..=self.lat_deg_polf2).suffix("°"));
        ui.end_row();
        ui.label("Maximal longitude degradation:");
        ui.add(
            egui::Slider::new(&mut self.lon_deg_polf1, self.lon_deg_min_b_int..=360.0).suffix("°"),
        );
        ui.end_row();
        ui.label("Maximal latitude degradation:");
        ui.add(egui::Slider::new(&mut self.lat_deg_polf2, self.lat_deg_min_pa..=90.0).suffix("°"));
        ui.end_row();
    }

    fn radius_builder(&mut self, ui: &mut Ui) {
        ui.label("Radius:");
        ui.add(egui::Slider::new(&mut self.radius_a, 0.0..=180.0).suffix("°"));
        ui.end_row();
    }

    fn radii_builder(&mut self, ui: &mut Ui) {
        ui.label("Internal radius:");
        ui.add(egui::Slider::new(&mut self.lon_deg_min_b_int, 0.0..=self.radius_a).suffix("°"));
        ui.end_row();
        ui.label("External radius:");
        ui.add(egui::Slider::new(&mut self.radius_a, self.lon_deg_min_b_int..=180.0).suffix("°"));
        ui.end_row();
    }

    fn degs_builder(&mut self, ui: &mut Ui) {
        ui.label("A degradation:");
        ui.add(egui::Slider::new(&mut self.radius_a, 0.0..=90.0).suffix("°"));
        ui.end_row();
        ui.label("B degradation:");
        ui.add(egui::Slider::new(&mut self.lon_deg_min_b_int, 0.0..=self.radius_a).suffix("°"));
        ui.end_row();
        ui.label("PA degradation:");
        ui.add(egui::Slider::new(&mut self.lat_deg_min_pa, 0.0..=90.0).suffix("°"));
        ui.end_row();
    }

    fn threshold_builder(&mut self, ui: &mut Ui) {
        ui.label("From Threshold :");
        ui.add(
            egui::Slider::new(&mut self.from_threshold, 0.0..=self.to_threshold).logarithmic(true),
        );
        ui.end_row();
        ui.label("To Threshold:");
        ui.add(egui::Slider::new(&mut self.to_threshold, 0.0..=1.0).logarithmic(true));
        ui.end_row();
    }

    //////////////////////////
    // Useful csv functions //

    #[cfg(target_arch = "wasm32")]
    fn load_csv(&mut self, typ: CreationType) -> Result<(), String> {
        let task = AsyncFileDialog::new()
            .add_filter("MOCs", &["csv"])
            .pick_file();

        let depth = self.depth;
        let mut name = format!("{}_{}", typ, self.depth);
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

    #[cfg(not(target_arch = "wasm32"))]
    fn load_csv(&mut self, typ: CreationType) -> Result<(), String> {
        if let Some(path) = FileDialog::new().add_filter("MOCs", &["csv"]).pick_file() {
            let depth = self.depth;
            let mut name = format!("{}_{}", typ, self.depth);
            if !self.name.is_empty() {
                name = self.name.clone();
            }
            let complement = self.comp;

            let mut file = File::open(&path).map_err(|_| "Error while opening file".to_string())?;
            let mut file_content = Vec::default();
            file.read_to_end(&mut file_content)
                .map_err(|e| format!("Error while reading file: {}", e))?;
            let file_content = unsafe { String::from_utf8_unchecked(file_content) };

            let _ = match typ {
                CreationType::Box => todo!(),
                CreationType::Cone => todo!(),
                CreationType::Coo => from_coo(&name, depth, file_content),
                CreationType::DecimalJd => from_decimal_jd(&name, depth, file_content),
                CreationType::DecimalJdRange => from_decimal_jd_range(&name, depth, file_content),
                CreationType::EllipticalCone => todo!(),
                CreationType::LargeCone => from_large_cones(&name, depth, file_content),
                CreationType::Polygon => from_polygon(&name, depth, file_content, complement),
                CreationType::Ring => todo!(),
                CreationType::SmallCone => from_small_cones(&name, depth, file_content),
                CreationType::ValuedCells => todo!(),
                CreationType::Zone => todo!(),
            };
        }
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
fn execute<F: std::future::Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
