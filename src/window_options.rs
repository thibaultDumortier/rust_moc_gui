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
    pub fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        let mut window = egui::Window::new("Test")
            .id(egui::Id::new("demo_window_options")) // required since we change the title
            .resizable(true)
            .title_bar(true)
            .scroll2([false,false])
            .enabled(true);
        window = window.open(open);
        window.show(ctx, |ui| self.ui(ui));
    }
    
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("A test window");
    }
}