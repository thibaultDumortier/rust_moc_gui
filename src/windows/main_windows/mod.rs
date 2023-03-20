pub(crate) mod creationui;
pub(crate) mod index;
pub(crate) mod multiple;
pub(crate) mod unitary;

use std::collections::BTreeSet;

use egui::{Context, ScrollArea, Ui};

use crate::windows::Window;
use creationui::CreationUis;
use multiple::MultipleUi;
use unitary::UnitaryUi;

pub struct MainUis {
    mainuis: Vec<Box<dyn Window>>,
    open: BTreeSet<String>,
}
impl Default for MainUis {
    fn default() -> Self {
        MainUis::from_main_uis(vec![
            Box::new(CreationUis::default()),
            Box::new(UnitaryUi::default()),
            Box::new(MultipleUi::default()),
        ])
    }
}
impl MainUis {
    pub fn from_main_uis(mainuis: Vec<Box<dyn Window>>) -> Self {
        let open = BTreeSet::new();
        Self { mainuis, open }
    }

    pub fn checkboxes(&mut self, ui: &mut Ui) {
        let Self {
            mainuis,
            open,
        } = self;
        for mainui in mainuis {
            let mut is_open = open.contains(mainui.name());
            ui.toggle_value(&mut is_open, mainui.name());
            set_open(open, mainui.name(), is_open);
        }
    }

    pub fn windows(&mut self, ctx: &Context) {
        let Self {
            mainuis,
            open,
        } = self;
        for mainui in mainuis {
            let mut is_open = open.contains(mainui.name());
            mainui.show(ctx, &mut is_open);
            set_open(open, mainui.name(), is_open);
        }
    }
}

// -----------------------------------------------------------

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

// -----------------------------------------------------------

pub struct MainUiWindows {
    mainuis: MainUis,
}

impl Default for MainUiWindows {
    fn default() -> Self {
        Self {
            mainuis: Default::default(),
        }
    }
}

impl MainUiWindows {
    /// Show the app ui (menu bar and windows).
    pub fn ui(&mut self, ctx: &Context) {
        self.desktop_ui(ctx);
    }

    fn desktop_ui(&mut self, ctx: &Context) {
        egui::SidePanel::right("Tools_panel")
            .resizable(false)
            .exact_width(200.0)
            .show(ctx, |ui| {
                egui::trace!(ui);
                ui.vertical_centered(|ui| {
                    ui.heading("Tools");
                });

                ui.separator();

                self.mainui_list_ui(ui);
            });

        self.show_windows(ctx);
    }

    /// Show the open windows.
    fn show_windows(&mut self, ctx: &Context) {
        self.mainuis.windows(ctx);
    }

    fn mainui_list_ui(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                self.mainuis.checkboxes(ui);
            });
        });
    }
}
