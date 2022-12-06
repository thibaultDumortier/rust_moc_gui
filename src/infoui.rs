#[derive(Clone, PartialEq, Default, PartialOrd, Ord, Eq)]
pub struct InfoWindow { 
    pub title: String
}

impl InfoWindow {
    pub fn new(title: String) -> Self{
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
        ui.label("A test window");
    }
}
