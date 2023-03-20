use std::borrow::Borrow;

use egui::Color32;
use moc::storage::u64idx::{common::MocQType, U64MocStore};

use crate::{
    utils::{
        commons::{fmt_qty, to_file},
        namestore::get_name,
    },
    windows::{View, Window},
};

#[derive(Clone, PartialEq, Default, Eq)]
pub struct InfoWindow {
    pub id: usize,
    texture: Option<egui::TextureHandle>,
    size: usize,
    info: String,
    name: String,
}
impl Window for InfoWindow {
    fn name(&self) -> &'static str {
        let s: &'static str = Box::leak(self.name.to_string().into_boxed_str());
        s
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

impl View for InfoWindow {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let qty = U64MocStore.get_qty_type(self.id).unwrap();

        ui.horizontal(|ui| {
            ui.label("MOC type:");
            ui.label(fmt_qty(qty));
        });

        //ui.add(egui::Slider::new(&mut self.size, 0..=150));

        match qty {
            MocQType::Space => {
                ui.label(&self.info);
                let texture = &self.texture.clone().unwrap();
                ui.add(egui::Image::new(texture, texture.size_vec2()).bg_fill(Color32::WHITE));
                if ui.button("Download image").clicked() {
                    let _ = to_file(
                        &get_name(self.id).unwrap(),
                        ".png",
                        "image/x-png",
                        U64MocStore.to_png(self.id, 300).unwrap(),
                    );
                }
            }
            MocQType::Time => {
                ui.label(&self.info);
            }
            MocQType::TimeSpace => {
                ui.label(&self.info);
            }
            MocQType::Frequency => unreachable!(),
        };
    }
}

impl InfoWindow {
    pub fn new(ctx: &egui::Context, id: usize) -> Result<Self, String> {
        let mut texture: Option<egui::TextureHandle> = None;
        if let Ok(i) = U64MocStore.to_image(id, 150) {
            texture =
                // Load the texture only once.
                Some(ctx.load_texture(
                    "moc_img",
                    egui::ColorImage::from_rgba_unmultiplied([300, 150], i.borrow()),
                    Default::default(),
                ));
        }

        let mut info = String::default();
        match U64MocStore.get_qty_type(id) {
            Ok(qty) => match qty {
                MocQType::Space => {
                    if let Ok(s) = U64MocStore.get_smoc_depth(id) {
                        info = format!(
                            "Depth: {}, Coverage: {}",
                            s.to_string(),
                            U64MocStore.get_coverage_percentage(id).unwrap().to_string()
                        )
                    }
                }
                MocQType::Time => {
                    if let Ok(t) = U64MocStore.get_tmoc_depth(id) {
                        info = format!("Depth: {}", t.to_string())
                    }
                }
                MocQType::Frequency => {
                    return Err(String::from("Frequency MOCs are not supported"))
                }
                MocQType::TimeSpace => {
                    if let Ok(st) = U64MocStore.get_stmoc_depths(id) {
                        info = format!(
                            "Depth S: {}\nDepth T: {}",
                            st.0.to_string(),
                            st.1.to_string()
                        )
                    }
                }
            },
            Err(e) => return Err(e),
        }

        let name = get_name(id).unwrap();

        Ok(Self {
            id,
            texture,
            info,
            name,
            size: 150,
        })
    }
}
