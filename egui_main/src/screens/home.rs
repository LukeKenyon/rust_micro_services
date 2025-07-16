use egui::{Context, Ui};

#[derive(Default)]
pub struct HomeScreen {
    // any settings fields here
}

impl HomeScreen {
    pub fn show(&mut self, _ctx: &Context, ui: &mut Ui) {
        ui.heading("Home");
        ui.label("home Screen.");
        // add toggles, selects, etc.
    }
}
