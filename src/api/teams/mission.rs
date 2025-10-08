use egui::Image;
use serde::Deserialize;

use crate::Units;

#[derive(Debug, Deserialize, Clone, Hash)]
pub struct Mission {
    /// the ID of a mission
    ///
    /// used by in-game orders to
    /// CTRL + F find the correct team
    /// or using search
    pub id: String,
    /// Name of the Mission
    name: String,
    /// list of unit IDs which form the
    /// team used for this mission
    team: Vec<String>,
    /// additional note giving information
    /// about this mission
    note: String,
}

impl Mission {
    pub fn render(&self, ui: &mut egui::Ui, units: &Units, origin: &str) {
        // TODO center and slightly increase font size
        ui.horizontal(|ui| {
            ui.label(format!("{} ({})", self.name, self.id));
        });

        if self.name != "Fleet" {
            ui.horizontal(|ui| {
                for unit in &self.team {
                    let unit = &units[unit.as_str()];
                    ui.vertical(|ui| {
                        // TODO check sizing once image finally load
                        ui.add(
                            Image::new(unit.image(origin))
                                .alt_text(&unit.name)
                                .shrink_to_fit()
                                .max_size(egui::Vec2::new(50., 50.)),
                        );
                        // TODO center name below image
                        ui.label(&unit.name);
                    });
                }
            });
        } else {
            ui.label("// TODO mimic in game fleet layout");
        }

        ui.horizontal(|ui| {
            ui.label(&self.note);
        });
    }
}
