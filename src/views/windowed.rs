use std::collections::BTreeSet;

use egui::{Context, ScrollArea, Ui};

use super::{creationui::CreationUis, multiple::MultipleUi, unitary::UnitaryUi, SubUi};

pub struct SubUis {
    subuis: Vec<Box<dyn SubUi>>,
    open: BTreeSet<String>,
}
impl Default for SubUis {
    fn default() -> Self {
        SubUis::from_sub_uis(vec![
            Box::new(CreationUis::default()),
            Box::new(UnitaryUi::default()),
            Box::new(MultipleUi::default()),
        ])
    }
}
impl SubUis {
    pub fn from_sub_uis(subuis: Vec<Box<dyn SubUi>>) -> Self {
        let open = BTreeSet::new();
        Self { subuis, open }
    }

    pub fn checkboxes(&mut self, ui: &mut Ui) {
        let Self { subuis, open } = self;
        for subui in subuis {
            let mut is_open = open.contains(subui.name());
            ui.toggle_value(&mut is_open, subui.name());
            set_open(open, subui.name(), is_open);
        }
    }

    pub fn windows(&mut self, ctx: &Context) {
        let Self { subuis, open } = self;
        for subui in subuis {
            let mut is_open = open.contains(subui.name());
            subui.show(ctx, &mut is_open);
            set_open(open, subui.name(), is_open);
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

pub struct SubUiWindows {
    subuis: SubUis,
}

impl Default for SubUiWindows {
    fn default() -> Self {
        Self {
            subuis: Default::default(),
        }
    }
}

impl SubUiWindows {
    /// Show the app ui (menu bar and windows).
    pub fn ui(&mut self, ctx: &Context) {
        self.desktop_ui(ctx);
    }

    fn desktop_ui(&mut self, ctx: &Context) {
        egui::SidePanel::right("egui_demo_panel")
            .resizable(false)
            .exact_width(200.0)
            .show(ctx, |ui| {
                egui::trace!(ui);
                ui.vertical_centered(|ui| {
                    ui.heading("Tools");
                });

                ui.separator();

                self.subui_list_ui(ui);
            });

        self.show_windows(ctx);
    }

    /// Show the open windows.
    fn show_windows(&mut self, ctx: &Context) {
        self.subuis.windows(ctx);
    }

    fn subui_list_ui(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                self.subuis.checkboxes(ui);

                if ui.button("Organize windows").clicked() {
                    ui.ctx().memory_mut(|mem| mem.reset_areas());
                }
            });
        });
    }
}
