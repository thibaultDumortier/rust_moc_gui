pub(crate) mod list_window;
pub(crate) mod main_windows;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

// Something to view
pub trait Window {
    // `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    // Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
