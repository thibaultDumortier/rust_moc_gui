use crate::{commons::Qty, loaders::store};

#[derive(Clone, PartialEq, Default, PartialOrd, Ord, Eq)]
pub struct InfoWindow {
    pub title: String,
}

impl InfoWindow {
    pub fn new(title: String) -> Self {
        Self { title }
    }

    pub fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        let mut window = egui::Window::new(self.title.clone())
            .id(egui::Id::new(self.title.clone())) // required since we change the title
            .resizable(true)
            .title_bar(true)
            .enabled(true);
        window = window.open(open);
        window.show(ctx, |ui| self.ui(ui));
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        let qty = store::get_qty(&self.title).unwrap();

        ui.horizontal(|ui| {
            ui.label("MOC type:");
            ui.label(qty.to_string().as_str());
        });

        match qty {
            Qty::Space => ui.label("Possible operations include:\n-All solo operations.\n-All same type duo operations.\n-SFold with a SpaceTime MOC."),
            Qty::Time => ui.label("Possible operations include:\n-Complement and degrade.\n-All same type duo operations\n-TFold with a SpaceTime MOC."),
            Qty::Timespace => ui.label("Possible operations include:\n-No solo operations.\n-All same type duo operations.\n-SFold or TFold depending on the other MOC's type."),
        };
    }
}
