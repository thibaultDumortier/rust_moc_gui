use crate::controllers::creation::*;
use crate::utils::namestore::add;

use super::creationui::CreationType;
use crate::windows::{View, Window};
use eframe::egui;
use egui::{TextEdit, Ui};
use eq_float::F64;

use moc::storage::u64idx::U64MocStore;
#[cfg(target_arch = "wasm32")]
use rfd::AsyncFileDialog;

#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;
use std::fs::File;
use std::io::Read;

#[derive(Default, Clone, Eq, PartialEq)]
pub struct CreationUis {
    name: String,
    depth: u8,
    lon_deg_polf1: F64,
    lat_deg_polf2: F64,
    radius_a: F64,
    lon_deg_min_b_int: F64,
    lat_deg_min_pa: F64,
    comp: bool,
    typ: CreationType,
    error: Option<String>,
    // Valued cells requirements
    density: bool,
    asc: bool,
    not_strict: bool,
    split: bool,
    revese_recursive_descent: bool,
    from_threshold: F64,
    to_threshold: F64,
}
impl Window for CreationUis {
    fn name(&self) -> &'static str {
        "MOC creation"
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
impl View for CreationUis {
    ////////////////////////////////////////////////
    // MAIN UI (this uses the sub UIs seen later) //

    // #Definition
    //      Creation_ui, the main UI component for MOC creation
    // #Args
    //  *   `ui`: the egui UI that needs to show the given components
    fn ui(&mut self, ui: &mut Ui) {
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

        ui.add_space(20.0);

        // The small paragraph before the match sets a grid layout to have every element aligned
        egui::Grid::new("my_grid")
            .num_columns(2)
            .spacing([5.0, 4.0])
            .striped(false)
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
        // if self.error.is_some() {
        //     return Err(self.error.clone().unwrap());
        // }
        // Ok(())
    }
}
impl CreationUis {
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
        ui.add(TextEdit::singleline(&mut self.name).hint_text("Name"));
        ui.end_row();

        if ui.button("Create").clicked() {
            err = None;
            if self.name.is_empty() {
                self.name = format!("Cone_of_rad_{}", self.radius_a.to_string().as_str());
            }
            if let Ok(id) = U64MocStore
                .from_cone(
                    self.lon_deg_polf1.0,
                    self.lat_deg_polf2.0,
                    self.radius_a.0,
                    self.depth,
                    2,
                )
                .map_err(|e| err = Some(e))
            {
                if let Err(e) = add(&self.name, id) {
                    err = Some(e);
                }
            }
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
        ui.add(TextEdit::singleline(&mut self.name).hint_text("Name"));
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
            if let Ok(id) = U64MocStore
                .from_ring(
                    self.lon_deg_polf1.0,
                    self.lat_deg_polf2.0,
                    self.lon_deg_min_b_int.0,
                    self.radius_a.0,
                    self.depth,
                    2,
                )
                .map_err(|e| err = Some(e))
            {
                if let Err(e) = add(&self.name, id) {
                    err = Some(e);
                }
            }
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
            if let Ok(id) = U64MocStore
                .from_elliptical_cone(
                    self.lon_deg_polf1.0,
                    self.lat_deg_polf2.0,
                    self.radius_a.0,
                    self.lon_deg_min_b_int.0,
                    self.lat_deg_min_pa.0,
                    self.depth,
                    2,
                )
                .map_err(|e| err = Some(e))
            {
                if let Err(e) = add(&self.name, id) {
                    err = Some(e);
                }
            }
            self.name = String::default();
        }
        err
    }
    pub(crate) fn zone_ui(&mut self, ui: &mut Ui, e: &Option<String>) -> Option<String> {
        let mut err = e.to_owned();
        self.depth_builder(ui);
        self.lons_lats_builder(ui);

        ui.label("New MOC name :");
        ui.add(TextEdit::singleline(&mut self.name).hint_text("Name"));
        ui.end_row();

        if ui.button("Create").clicked() {
            err = None;
            if self.name.is_empty() {
                self.name = format!(
                    "Zone_deg_{}_{}",
                    self.lon_deg_min_b_int, self.lat_deg_min_pa
                );
            }
            if let Ok(id) = U64MocStore
                .from_zone(
                    self.lon_deg_min_b_int.0,
                    self.lat_deg_min_pa.0,
                    self.lon_deg_polf1.0,
                    self.lat_deg_polf2.0,
                    self.depth,
                )
                .map_err(|e| err = Some(e))
            {
                if let Err(e) = add(&self.name, id) {
                    err = Some(e);
                }
            }
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
            if let Ok(id) = U64MocStore
                .from_box(
                    self.lon_deg_polf1.0,
                    self.lat_deg_polf2.0,
                    self.radius_a.0,
                    self.lon_deg_min_b_int.0,
                    self.lat_deg_min_pa.0,
                    self.depth,
                )
                .map_err(|e| err = Some(e))
            {
                if let Err(e) = add(&self.name, id) {
                    err = Some(e)
                }
            }
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
        ui.add(TextEdit::singleline(&mut self.name).hint_text("Name"));
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

    // Jd_ui and Jdr_ui are different to the rest, they allow Time MOCs creation.
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

        ui.label("Density: ");
        ui.checkbox(&mut self.density, "");
        ui.end_row();
        ui.label("Asc: ");
        ui.checkbox(&mut self.asc, "");
        ui.end_row();
        ui.label("Strict: ");
        ui.checkbox(&mut self.not_strict, "");
        ui.end_row();
        ui.label("Split: ");
        ui.checkbox(&mut self.split, "");
        ui.end_row();
        ui.label("Revese recursive descent: ");
        ui.checkbox(&mut self.revese_recursive_descent, "");
        ui.end_row();
        ui.label("New MOC name :");
        ui.add(TextEdit::singleline(&mut self.name).hint_text("Name"));
        ui.end_row();

        if ui
            .button("Open coo file")
            .on_hover_text_at_pointer(
                "CSV file containing one coordinate per row:RA,DEC in decimal degrees",
            )
            .clicked()
        {
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
                    let id = from_valued_cells(
                        depth,
                        density,
                        from_threshold.0,
                        to_threshold.0,
                        asc,
                        not_strict,
                        split,
                        revese_recursive_descent,
                        file_content,
                    )
                    .unwrap();
                    add(&name, id);
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

        ui.label("Density: ");
        ui.checkbox(&mut self.density, "");
        ui.end_row();
        ui.label("Asc: ");
        ui.checkbox(&mut self.asc, "");
        ui.end_row();
        ui.label("Strict: ");
        ui.checkbox(&mut self.not_strict, "");
        ui.end_row();
        ui.label("Split: ");
        ui.checkbox(&mut self.split, "");
        ui.end_row();
        ui.label("Revese recursive descent: ");
        ui.checkbox(&mut self.revese_recursive_descent, "");
        ui.end_row();
        ui.label("New MOC name :");
        ui.add(TextEdit::singleline(&mut self.name).hint_text("Name"));
        ui.end_row();

        if ui
            .button("Open coo file")
            .on_hover_text_at_pointer(
                "CSV file containing one coordinate per row:RA,DEC in decimal degrees",
            )
            .clicked()
        {
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

                if let Ok(id) = from_valued_cells(
                    depth,
                    density,
                    from_threshold.0,
                    to_threshold.0,
                    asc,
                    not_strict,
                    split,
                    revese_recursive_descent,
                    file_content,
                )
                .map_err(|e| err = Some(e))
                {
                    if let Err(e) = add(&name, id) {
                        err = Some(e)
                    }
                }
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
        ui.add(TextEdit::singleline(&mut self.name).hint_text("Name"));
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
        ui.add(TextEdit::singleline(&mut self.name).hint_text("Name"));
        ui.end_row();

        if ui
            .button("Open coo file")
            .on_hover_text_at_pointer(
                "CSV file containing one coordinate per row:RA,DEC in decimal degrees",
            )
            .clicked()
        {
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
        ui.add(
            egui::Slider::new(&mut self.lon_deg_polf1.0, 0.0..=360.0)
                .suffix("°")
                .fixed_decimals(11),
        );
        ui.end_row();
        ui.label("Latitude degradation:");
        ui.add(
            egui::Slider::new(&mut self.lat_deg_polf2.0, -90.0..=90.0)
                .suffix("°")
                .fixed_decimals(11),
        );
        ui.end_row();
    }

    fn lons_lats_builder(&mut self, ui: &mut Ui) {
        ui.label("Minimal longitude degradation:");
        ui.add(
            egui::Slider::new(&mut self.lon_deg_min_b_int.0, 0.0..=self.lon_deg_polf1.0)
                .suffix("°")
                .fixed_decimals(11),
        );
        ui.end_row();
        ui.label("Minimal latitude degradation:");
        ui.add(
            egui::Slider::new(&mut self.lat_deg_min_pa.0, -90.0..=self.lat_deg_polf2.0)
                .suffix("°")
                .fixed_decimals(11),
        );
        ui.end_row();
        ui.label("Maximal longitude degradation:");
        ui.add(
            egui::Slider::new(&mut self.lon_deg_polf1.0, self.lon_deg_min_b_int.0..=360.0)
                .suffix("°")
                .fixed_decimals(11),
        );
        ui.end_row();
        ui.label("Maximal latitude degradation:");
        ui.add(
            egui::Slider::new(&mut self.lat_deg_polf2.0, self.lat_deg_min_pa.0..=90.0)
                .suffix("°")
                .fixed_decimals(11),
        );
        ui.end_row();
    }

    fn radius_builder(&mut self, ui: &mut Ui) {
        ui.label("Radius:");
        ui.add(
            egui::Slider::new(&mut self.radius_a.0, 0.0..=180.0)
                .suffix("°")
                .fixed_decimals(11),
        );
        ui.end_row();
    }

    fn radii_builder(&mut self, ui: &mut Ui) {
        ui.label("Internal radius:");
        ui.add(
            egui::Slider::new(&mut self.lon_deg_min_b_int.0, 0.0..=self.radius_a.0)
                .suffix("°")
                .fixed_decimals(11),
        );
        ui.end_row();
        ui.label("External radius:");
        ui.add(
            egui::Slider::new(&mut self.radius_a.0, self.lon_deg_min_b_int.0..=180.0)
                .suffix("°")
                .fixed_decimals(11),
        );
        ui.end_row();
    }

    fn degs_builder(&mut self, ui: &mut Ui) {
        ui.label("A degradation:");
        ui.add(
            egui::Slider::new(&mut self.radius_a.0, 0.0..=90.0)
                .suffix("°")
                .fixed_decimals(11),
        );
        ui.end_row();
        ui.label("B degradation:");
        ui.add(
            egui::Slider::new(&mut self.lon_deg_min_b_int.0, 0.0..=self.radius_a.0)
                .suffix("°")
                .fixed_decimals(11),
        );
        ui.end_row();
        ui.label("PA degradation:");
        ui.add(
            egui::Slider::new(&mut self.lat_deg_min_pa.0, 0.0..=90.0)
                .suffix("°")
                .fixed_decimals(11),
        );
        ui.end_row();
    }

    fn threshold_builder(&mut self, ui: &mut Ui) {
        ui.label("From Threshold :");
        ui.add(
            egui::Slider::new(&mut self.from_threshold.0, 0.0..=self.to_threshold.0)
                .logarithmic(true)
                .fixed_decimals(11),
        );
        ui.end_row();
        ui.label("To Threshold:");
        ui.add(
            egui::Slider::new(&mut self.to_threshold.0, 0.0..=1.0)
                .logarithmic(true)
                .fixed_decimals(11),
        );
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

                if let Ok(id) = match typ {
                    CreationType::Coo => from_coo(depth, file_content),
                    CreationType::DecimalJd => from_decimal_jd(depth, file_content),
                    CreationType::DecimalJdRange => from_decimal_jd_range(depth, file_content),
                    CreationType::LargeCone => from_large_cones(depth, file_content),
                    CreationType::Polygon => from_polygon(depth, file_content, complement),
                    CreationType::SmallCone => from_small_cones(depth, file_content),
                    _ => todo!(),
                } {
                    let _ = add(&name, id);
                }
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

            if let Ok(id) = match typ {
                CreationType::Coo => from_coo(depth, file_content),
                CreationType::DecimalJd => from_decimal_jd(depth, file_content),
                CreationType::DecimalJdRange => from_decimal_jd_range(depth, file_content),
                CreationType::LargeCone => from_large_cones(depth, file_content),
                CreationType::Polygon => from_polygon(depth, file_content, complement),
                CreationType::SmallCone => from_small_cones(depth, file_content),
                _ => todo!(),
            } {
                add(&name, id)?
            }
        }
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
fn execute<F: std::future::Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
