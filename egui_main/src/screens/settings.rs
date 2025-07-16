use egui::{Context, Ui};

#[derive(Default)]
pub struct SettingsScreen {
    // any settings fields here
}

impl SettingsScreen {
    pub fn show(&mut self, _ctx: &Context, ui: &mut Ui) {
        ui.heading("Settings");
        ui.label("Configure your preferences here.");
        // add toggles, selects, etc.
    }
}
