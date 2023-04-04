pub(crate) mod creationui;
pub(crate) mod multiple;
pub(crate) mod unitary;

use std::collections::BTreeSet;

use egui::{Context, Ui};

use crate::{windows::Window, utils::commons::set_open};
use creationui::CreationUis;
use multiple::MultipleUi;
use unitary::UnitaryUi;

pub struct MainWindows {
    mainuis: Vec<Box<dyn Window>>,
    open: BTreeSet<String>,
}
impl Default for MainWindows {
    fn default() -> Self {
        MainWindows::from_main_uis(vec![
            Box::<CreationUis>::default(),
            Box::<UnitaryUi>::default(),
            Box::<MultipleUi>::default(),
        ])
    }
}
impl MainWindows {
    pub fn from_main_uis(mainuis: Vec<Box<dyn Window>>) -> Self {
        let open = BTreeSet::new();
        Self { mainuis, open }
    }

    pub fn checkboxes(&mut self, ui: &mut Ui) {
        let Self { mainuis, open } = self;
        for mainui in mainuis {
            let mut is_open = open.contains(mainui.name());
            ui.toggle_value(&mut is_open, mainui.name());
            set_open(open, mainui.name(), is_open);
        }
    }

    pub fn windows(&mut self, ctx: &Context) {
        let Self { mainuis, open } = self;
        for mainui in mainuis {
            let mut is_open = open.contains(mainui.name());
            mainui.show(ctx, &mut is_open);
            set_open(open, mainui.name(), is_open);
        }
    }
}