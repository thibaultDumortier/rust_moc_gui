use super::{creationui::CreationUis, opui::OpUis};

#[derive(Clone, Eq, Hash, Debug)]
pub enum UiMenu {
    One,
    Two,
    Crea,
}
impl Default for UiMenu {
    fn default() -> Self {
        UiMenu::Crea
    }
}
impl PartialEq for UiMenu {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (UiMenu::One, UiMenu::One) | (UiMenu::Two, UiMenu::Two) | (UiMenu::Crea, UiMenu::Crea)
        )
    }
}

#[derive(Clone, Default, Eq)]
pub struct SubUiWindow {
    uitype: UiMenu,
    creation: CreationUis,
    opui: OpUis,
}
impl PartialEq for SubUiWindow {
    fn eq(&self, other: &Self) -> bool {
        match (self.clone(), other.clone()) {
            (a, b) => a.uitype == b.uitype,
        }
    }
}

impl SubUiWindow {
    pub fn new(uitype: UiMenu) -> Result<Self, String> {
        Ok(Self {
            uitype,
            creation: CreationUis::default(),
            opui: OpUis::default(),
        })
    }

    pub fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        let n = match &self.uitype {
            UiMenu::One => 0,
            UiMenu::Two => 1,
            UiMenu::Crea => 2,
        };

        let mut window = egui::Window::new(format!("{:?}", self.uitype).clone())
            .id(egui::Id::new(n)) // required since we change the title
            .resizable(false)
            .title_bar(true)
            .enabled(true);
        window = window.open(open);
        window.show(ctx, |ui| self.ui(ui));
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        let _ = match &self.uitype {
            UiMenu::One => self.opui.moc_op1(ui).map_err(|e| self.err(&e)),
            UiMenu::Two => self.opui.moc_op2(ui).map_err(|e| self.err(&e)),
            UiMenu::Crea => self.creation.creation_ui(ui).map_err(|e| self.err(&e)),
        };
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn err(&mut self, msg: &str) {
        use rfd::MessageDialog;

        let m = MessageDialog::new()
            .set_buttons(rfd::MessageButtons::Ok)
            .set_title("Error !")
            .set_description(msg);
        m.show();
    }

    #[cfg(target_arch = "wasm32")]
    fn err(&mut self, msg: &str) {
        use rfd::AsyncMessageDialog;

        let m = AsyncMessageDialog::new()
            .set_buttons(rfd::MessageButtons::Ok)
            .set_title("Error !")
            .set_description(msg);
        m.show();
    }
}
