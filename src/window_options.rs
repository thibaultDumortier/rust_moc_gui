#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct WindowOptions {
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self { }
    }
}

impl WindowOptions {
    pub fn show(&mut self, ctx: &egui::Context) {
        let Self {
        } = self.clone();

        let mut window = egui::Window::new("Test")
            .id(egui::Id::new("demo_window_options")) // required since we change the title
            .resizable(true)
            .collapsible(true)
            .title_bar(true)
            .scroll2([false,false])
            .enabled(true);
        window.show(ctx, |ui| self.ui(ui));
    }
}

impl WindowOptions {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let Self { } = self;

        ui.label("A test window");
    }
}