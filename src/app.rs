use crate::{Mission, Phase, Planet, Teams, Units};

pub struct App {
    units: Units,
    teams: Teams,
    search: String,

    #[cfg(target_arch = "wasm32")]
    window: web_sys::Window,
    #[cfg(target_arch = "wasm32")]
    origin: String,
}

// TODO replace the images in assets/ with custom made ones (adjust in manifest.json and index.html and check if used in other locations)

impl App {
    /// Called once before the first frame.
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        #[cfg(target_arch = "wasm32")] screen: web_sys::Window,
    ) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let units = toml::from_slice(include_bytes!("../assets/data/Units.toml"))
            .expect("failed to load units");

        let teams = Teams::load();

        Self {
            units,
            teams,
            search: Default::default(),
            #[cfg(target_arch = "wasm32")]
            window: screen,
            #[cfg(target_arch = "wasm32")]
            origin: if cc.integration_info.web_info.location.url.contains("dev") {
                cc.integration_info.web_info.location.origin.clone()
            } else {
                cc.integration_info.web_info.location.url.clone()
            },
        }
    }

    /// screen resolution (width, height) in pixels
    #[cfg(target_arch = "wasm32")]
    fn resolution(&self) -> (f32, f32) {
        (
            self.window
                .inner_width()
                .expect("missing width")
                .as_f64()
                .expect("is number") as f32,
            self.window
                .inner_height()
                .expect("missing height")
                .as_f64()
                .expect("is number") as f32,
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn resolution(&self, ctx: &egui::Context) -> (f32, f32) {
        let size = ctx.screen_rect();
        (size.max.x - size.min.x, size.max.y - size.min.y)
    }

    // TODO fns for different types of sizes (cap ship, starting ship, reinforcement icons, different kinds of texts, etc.), see example
    fn character_icon_size(
        &self,
        #[cfg(not(target_arch = "wasm32"))] ctx: &egui::Context,
    ) -> egui::Vec2 {
        let res = self.resolution(
            #[cfg(not(target_arch = "wasm32"))]
            ctx,
        );
        let base = if self.is_mobile() { res.0 } else { res.1 };
        let size = base / 20.;
        egui::Vec2::new(size, size)
    }

    /// true => portrait mode
    ///
    /// false => landscape mode
    #[cfg(target_arch = "wasm32")]
    fn is_mobile(&self) -> bool {
        let screen = self.window.screen().expect("missing screen");

        screen.avail_height().expect("missing avail height")
            > screen.avail_width().expect("missing avail height")
    }

    #[cfg(target_arch = "wasm32")]
    fn render_phase(&self, ui: &mut egui::Ui, phase: &Phase) {
        if self.is_mobile() {
            ui.horizontal(|ui| {
                self.render_planet(ui, &phase.dark);
                self.render_planet(ui, &phase.mixed);
                self.render_planet(ui, &phase.light);
            });
        } else {
            ui.columns(3, |ui| {
                self.render_planet(&mut ui[0], &phase.dark);
                self.render_planet(&mut ui[1], &phase.mixed);
                self.render_planet(&mut ui[2], &phase.light);
            });
        }
    }

    fn render_planet(&self, ui: &mut egui::Ui, planet: &Planet) {
        // TODO size based on resolution
        ui.vertical_centered(|ui| ui.heading(&planet.name));
        for mission in &planet.missions {
            ui.separator();
            self.render_mission(ui, mission);
        }
    }

    fn render_mission(&self, ui: &mut egui::Ui, mission: &Mission) {
        ui.vertical_centered(|ui| {
            // TODO size based on resolution
            ui.label(egui::RichText::new(format!("{} ({})", mission.name, mission.id)).size(20.));
        });

        ui.horizontal(|ui| {
            let mut missing;
            if mission.name != "Fleet" {
                missing = 5;
                for unit in &mission.team {
                    missing -= 1;
                    let unit = &self.units[unit.as_str()];
                    ui.vertical(|ui| {
                        ui.add_sized(
                            self.character_icon_size(),
                            |ui: &mut egui::Ui| -> egui::Response {
                                ui.add(
                                    egui::Image::new(unit.image(&self.origin))
                                        .alt_text(&unit.name)
                                        .shrink_to_fit(),
                                );
                                // TODO center name below image and make it bigger
                                ui.vertical_centered(|ui| {
                                    ui.label(&unit.name);
                                })
                                .response
                            },
                        );
                    });
                }
            } else {
                missing = 8;
                for (idx, ship) in mission.team.iter().enumerate() {
                    missing -= 1;
                    let ship = &self.units[ship.as_str()];

                    let image = egui::Image::new(ship.image(&self.origin)).alt_text(&ship.name);
                    ui.vertical(|ui| {
                        ui.add(image);
                        ui.label(&ship.name);
                    });

                    if idx == 0 {
                        // TODO cap ship biggest icon
                    } else if idx < 4 {
                        // TODO starting lineup slightly smaller, aligned to the top
                    } else {
                        // TODO reinforcements again slightly small and "Reinforcements" aligned to the top and icons below
                    }
                }
            }
            for _ in 0..missing {
                // TODO add a placeholder for empty slots?
                ui.label("[PH]");
            }
        });

        ui.horizontal(|ui| {
            // TODO bigger text?
            ui.label(format!("Note: {}", mission.note));
        });
    }
}

impl eframe::App for App {
    // TODO include native support
    #[cfg(not(target_arch = "wasm32"))]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let _: &Units = &self.units;
        let _: &Teams = &self.teams;
        let _: &String = &self.search;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("currently only wasm32 is supported");

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
            .show(ctx, |ui| ui.heading("Rise of the Empire TB Team setup"));

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
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
                                self.render_mission(ui, mission);
                            }
                        }
                    });
                });

                for (idx, phase) in self.teams.phases.iter().enumerate() {
                    // TODO size based on resolution
                    ui.collapsing(format!("Phase {}", idx + 1), |ui| {
                        // phase.render(ui, &self.units, origin);
                        self.render_phase(ui, phase);
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
    // TODO add links to me (and guild?)
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
