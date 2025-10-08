use crate::{Mission, Units};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Planet {
    pub name: String,
    #[serde(rename = "mission")]
    pub missions: Vec<Mission>,
}

impl Planet {
    pub fn render(&self, ui: &mut egui::Ui, units: &Units, origin: &str) {
        ui.vertical_centered(|ui| ui.heading(&self.name));
        for mission in &self.missions {
            ui.separator();
            mission.render(ui, units, origin);
        }
    }
}
