use crate::{Teams, Units};

pub struct App {
    units: Units,
    teams: Teams,
    search: String,
}

// TODO replace the images in assets/ with custom made ones (adjust in manifest.json and index.html and check if used in other locations)

impl Default for App {
    fn default() -> Self {
        let units = toml::from_slice(include_bytes!("../assets/data/Units.toml"))
            .expect("failed to load units");

        let teams = Teams::load();

        Self {
            units,
            teams,
            search: Default::default(),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        egui_extras::install_image_loaders(&cc.egui_ctx);
        // cc.egui_ctx.add_image_loader(loader);

        Default::default()
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
            .show(ctx, |ui| ui.heading("Rise of the Empire TB Team setup"));

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let origin = &frame.info().web_info.location.origin;
                // TODO remove
                ui.label(origin.clone());

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Search:");
                        ui.text_edit_singleline(&mut self.search);
                        if ui.button("Clear").clicked() {
                            self.search = String::new();
                        }
                    });
                    ui.vertical(|ui| {
                        if !self.search.is_empty() {
                            let teams = self.teams.search(&self.search);
                            for (idx, mission) in teams.iter().enumerate() {
                                if idx > 0 {
                                    ui.separator();
                                }
                                mission.render(ui, &self.units, origin);
                            }
                        }
                    });
                });

                for (idx, phase) in self.teams.phases.iter().enumerate() {
                    ui.collapsing(format!("Phase {}", idx + 1), |ui| {
                        phase.render(ui, &self.units, origin);
                    });
                }

                ui.separator();

                ui.add(egui::github_link_file!(
                    "https://github.com/ArckyPN/swgoh-tb",
                    "Source code."
                ));

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    powered_by_egui_and_eframe(ui);
                    egui::warn_if_debug_build(ui);
                });
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
