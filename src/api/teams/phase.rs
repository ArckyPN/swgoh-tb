use crate::{Planet, Units};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Phase {
    #[serde(rename = "Dark")]
    pub dark: Planet,
    #[serde(rename = "Mixed")]
    pub mixed: Planet,
    #[serde(rename = "Light")]
    pub light: Planet,
}

impl Phase {
    pub fn render(&self, ui: &mut egui::Ui, units: &Units, origin: &str) {
        // TODO figure out how to get device type / mobile or desktop version to either show all three planets in columns or one after the other
        ui.columns(3, |ui| {
            self.dark.render(&mut ui[0], units, origin);
            self.mixed.render(&mut ui[1], units, origin);
            self.light.render(&mut ui[2], units, origin);
        });
    }
}
