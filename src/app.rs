use crate::{Mission, Phase, Planet, Teams, Unit, Units};

pub struct App {
    units: Units,
    teams: Teams,
    search: String,
    info: bool,

    #[cfg(target_arch = "wasm32")]
    window: web_sys::Window,
    origin: String,
}

// TODO replace the images in assets/ with custom made ones (adjust in manifest.json and index.html and check if used in other locations)
// TODO figure out how to include Omicrons, as icons or just in notes?

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
            info: false,
            #[cfg(target_arch = "wasm32")]
            window: screen,
            #[cfg(target_arch = "wasm32")]
            origin: if cc.integration_info.web_info.location.url.contains("dev") {
                cc.integration_info.web_info.location.origin.clone()
            } else {
                cc.integration_info.web_info.location.url.clone()
            },
            #[cfg(not(target_arch = "wasm32"))]
            origin: "../assets".to_owned(),
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

    #[expect(clippy::unused_self)]
    #[cfg(not(target_arch = "wasm32"))]
    fn resolution(&self, ctx: &egui::Context) -> (f32, f32) {
        let size = ctx.screen_rect();
        (size.max.x - size.min.x, size.max.y - size.min.y)
    }

    fn reference_size(&self, #[cfg(not(target_arch = "wasm32"))] ctx: &egui::Context) -> f32 {
        let res = self.resolution(
            #[cfg(not(target_arch = "wasm32"))]
            ctx,
        );
        if self.is_mobile(
            #[cfg(not(target_arch = "wasm32"))]
            ctx,
        ) {
            res.0
        } else {
            res.1
        }
    }

    fn character_icon_size(
        &self,
        #[cfg(not(target_arch = "wasm32"))] ctx: &egui::Context,
    ) -> egui::Vec2 {
        let base = self.reference_size(
            #[cfg(not(target_arch = "wasm32"))]
            ctx,
        );
        let size = base / 20.;
        egui::Vec2::new(size, size)
    }

    fn planet_font_size(&self, #[cfg(not(target_arch = "wasm32"))] ctx: &egui::Context) -> f32 {
        self.reference_size(
            #[cfg(not(target_arch = "wasm32"))]
            ctx,
        ) / 40.
    }

    fn mission_font_size(&self, #[cfg(not(target_arch = "wasm32"))] ctx: &egui::Context) -> f32 {
        self.reference_size(
            #[cfg(not(target_arch = "wasm32"))]
            ctx,
        ) / 60.
    }

    fn note_font_size(&self, #[cfg(not(target_arch = "wasm32"))] ctx: &egui::Context) -> f32 {
        self.reference_size(
            #[cfg(not(target_arch = "wasm32"))]
            ctx,
        ) / 70.
    }

    fn unit_font_size(&self, #[cfg(not(target_arch = "wasm32"))] ctx: &egui::Context) -> f32 {
        self.reference_size(
            #[cfg(not(target_arch = "wasm32"))]
            ctx,
        ) / 135.
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

    #[cfg(not(target_arch = "wasm32"))]
    fn is_mobile(&self, ctx: &egui::Context) -> bool {
        let res = self.resolution(ctx);
        res.1 > res.0
    }

    fn render_phase(
        &self,
        ui: &mut egui::Ui,
        phase: &Phase,
        #[cfg(not(target_arch = "wasm32"))] ctx: &egui::Context,
    ) {
        if self.is_mobile(
            #[cfg(not(target_arch = "wasm32"))]
            ctx,
        ) {
            // TODO mobile support
        } else {
            match &phase.bonus {
                Some(bonus) => {
                    ui.columns(4, |ui| {
                        let mut ui = ui.iter_mut();
                        self.render_planet(
                            ui.next().expect("has one"),
                            &phase.dark,
                            #[cfg(not(target_arch = "wasm32"))]
                            ctx,
                        );
                        if bonus.name.eq_ignore_ascii_case("Mandalore") {
                            self.render_planet(
                                ui.next().expect("has one"),
                                bonus,
                                #[cfg(not(target_arch = "wasm32"))]
                                ctx,
                            );
                        }
                        self.render_planet(
                            ui.next().expect("has one"),
                            &phase.mixed,
                            #[cfg(not(target_arch = "wasm32"))]
                            ctx,
                        );
                        if bonus.name.eq_ignore_ascii_case("Zeffo") {
                            self.render_planet(
                                ui.next().expect("has one"),
                                bonus,
                                #[cfg(not(target_arch = "wasm32"))]
                                ctx,
                            );
                        }
                        self.render_planet(
                            ui.next().expect("has one"),
                            &phase.dark,
                            #[cfg(not(target_arch = "wasm32"))]
                            ctx,
                        );
                    });
                }
                None => {
                    ui.columns_const(|[c1, c2, c3]| {
                        self.render_planet(
                            c1,
                            &phase.dark,
                            #[cfg(not(target_arch = "wasm32"))]
                            ctx,
                        );
                        self.render_planet(
                            c2,
                            &phase.mixed,
                            #[cfg(not(target_arch = "wasm32"))]
                            ctx,
                        );
                        self.render_planet(
                            c3,
                            &phase.light,
                            #[cfg(not(target_arch = "wasm32"))]
                            ctx,
                        );
                    });
                }
            }
        }
    }

    fn render_planet(
        &self,
        ui: &mut egui::Ui,
        planet: &Planet,
        #[cfg(not(target_arch = "wasm32"))] ctx: &egui::Context,
    ) {
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new(&planet.name).size(self.planet_font_size(
                    #[cfg(not(target_arch = "wasm32"))]
                    ctx,
                )),
            );
        });
        for mission in &planet.missions {
            ui.separator();
            self.render_mission(
                ui,
                mission,
                #[cfg(not(target_arch = "wasm32"))]
                ctx,
            );
        }
    }

    fn render_mission(
        &self,
        ui: &mut egui::Ui,
        mission: &Mission,
        #[cfg(not(target_arch = "wasm32"))] ctx: &egui::Context,
    ) {
        ui.vertical(|ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(format!("{} ({})", mission.name, mission.id)).size(
                        self.mission_font_size(
                            #[cfg(not(target_arch = "wasm32"))]
                            ctx,
                        ),
                    ),
                );
            });

            ui.label(
                egui::RichText::new(if mission.name != "Fleet" {
                    format!("Relic: {}+", mission.relic)
                } else {
                    "7* Stars".to_owned()
                })
                .size(self.note_font_size(
                    #[cfg(not(target_arch = "wasm32"))]
                    ctx,
                )),
            );

            ui.horizontal(|ui| {
                let missing = if mission.name != "Fleet" {
                    self.render_squad(
                        ui,
                        &mission.team,
                        #[cfg(not(target_arch = "wasm32"))]
                        ctx,
                    )
                } else {
                    self.render_fleet(
                        ui,
                        &mission.team,
                        #[cfg(not(target_arch = "wasm32"))]
                        ctx,
                    )
                };
                self.missing_helper(
                    missing,
                    ui,
                    #[cfg(not(target_arch = "wasm32"))]
                    ctx,
                );
            });

            ui.label(egui::RichText::new(format!("Note: {}", mission.note)).size(
                self.note_font_size(
                    #[cfg(not(target_arch = "wasm32"))]
                    ctx,
                ),
            ));
        });
    }

    /// simple helper to make sure all slots are filled in the PhaseX.toml files
    fn missing_helper(
        &self,
        missing: i32,
        ui: &mut egui::Ui,
        #[cfg(not(target_arch = "wasm32"))] ctx: &egui::Context,
    ) {
        for _ in 0..missing {
            ui.vertical(|ui| {
                self.render_unit(
                    ui,
                    &Unit::forgot(),
                    self.character_icon_size(
                        #[cfg(not(target_arch = "wasm32"))]
                        ctx,
                    ),
                    #[cfg(not(target_arch = "wasm32"))]
                    ctx,
                );
            });
        }
    }

    fn render_unit(
        &self,
        ui: &mut egui::Ui,
        unit: &Unit,
        size: impl Into<egui::Vec2>,
        #[cfg(not(target_arch = "wasm32"))] ctx: &egui::Context,
    ) {
        ui.vertical(|ui| {
            ui.add_sized(size, |ui: &mut egui::Ui| -> egui::Response {
                let res = ui.add(egui::Image::new(unit.image(&self.origin)).shrink_to_fit());
                if unit.id.eq_ignore_ascii_case("[ph]") {
                    res.on_hover_text("open spots can be filled with whatever you want, but generally these spots are not needed");
                } else if unit.id.eq_ignore_ascii_case("unavailable") {
                    res.on_hover_text("this spot cannot be filled");
                }
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new(&unit.name).size(self.unit_font_size(#[cfg(not(target_arch = "wasm32"))] ctx)));
                })
                .response
            });
        });
    }

    fn render_squad(
        &self,
        ui: &mut egui::Ui,
        team: &[String],
        #[cfg(not(target_arch = "wasm32"))] ctx: &egui::Context,
    ) -> i32 {
        let mut missing = 5;

        ui.horizontal(|ui| {
            for unit in team {
                missing -= 1;
                let unit = self.units.get(unit);
                self.render_unit(
                    ui,
                    &unit,
                    self.character_icon_size(
                        #[cfg(not(target_arch = "wasm32"))]
                        ctx,
                    ),
                    #[cfg(not(target_arch = "wasm32"))]
                    ctx,
                );
            }
        });

        missing
    }

    fn render_fleet(
        &self,
        ui: &mut egui::Ui,
        team: &[String],
        #[cfg(not(target_arch = "wasm32"))] ctx: &egui::Context,
    ) -> i32 {
        let mut missing = 8;

        let mut team = team.iter().map(|u| self.units.get(u));

        ui.horizontal(|ui| {
            // capital ship
            let cap = team.next().expect("must have capital ship");
            self.render_unit(
                ui,
                &cap,
                self.character_icon_size(
                    #[cfg(not(target_arch = "wasm32"))]
                    ctx,
                ) * 1.1,
                #[cfg(not(target_arch = "wasm32"))]
                ctx,
            );
            missing -= 1;

            // starting lineup
            for starting in team.by_ref() {
                self.render_unit(
                    ui,
                    &starting,
                    self.character_icon_size(
                        #[cfg(not(target_arch = "wasm32"))]
                        ctx,
                    ) * 0.9,
                    #[cfg(not(target_arch = "wasm32"))]
                    ctx,
                );
                missing -= 1;
                if missing == 4 {
                    break;
                }
            }

            // reinforcements
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new("Reinforcements").size(self.unit_font_size(
                        #[cfg(not(target_arch = "wasm32"))]
                        ctx,
                    )),
                );
                ui.horizontal(|ui| {
                    for reinforcement in team {
                        self.render_unit(
                            ui,
                            &reinforcement,
                            self.character_icon_size(
                                #[cfg(not(target_arch = "wasm32"))]
                                ctx,
                            ) * 0.8,
                            #[cfg(not(target_arch = "wasm32"))]
                            ctx,
                        );
                        missing -= 1;
                    }
                });
            });
        });

        missing
    }

    fn render_info(ui: &mut egui::Ui, font_size: f32) {
        let text = |text: &str| egui::RichText::new(text).size(font_size);

        ui.horizontal(|ui| {
            ui.label(text(
                "This website provides ideal teams to master the Territory Battle ",
            ));
            ui.label(text("Rise of the Empire").monospace());
            ui.label(text(" in Star Wars: Galaxy of Heroes."));
        });

        ui.add_space(20.);

        ui.horizontal(|ui| {
            ui.label(text("The information is based on a combination of personal experience by myself and my guild and the websites "));
            ui.hyperlink_to(text("https://genskaar.github.io/tb_empire/").monospace(), "https://genskaar.github.io/tb_empire/");
            ui.label(text(" and "));
            ui.hyperlink_to(text("https://www.swgohrote.com/").monospace(), "https://www.swgohrote.com/");
            ui.label(".");
        });

        ui.add_space(20.);

        ui.horizontal(|ui| {
            ui.label(text("The major contribution I am bringing, which the other sites aren't doing, is providing a recommendation of teams to use without any overlap per phase. This way one is able to easily look up teams to clear a full phase or get an idea on which teams to build up."));
        });

        ui.add_space(20.);

        ui.horizontal(|ui| {
            ui.label("Where possible, I prefer teams which are able to full auto missions (without Omicrons). When full auto is not possible, I will give alternatives.");
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Rise of the Empire TB Team setup");
                if ui.button("Info").clicked() {
                    self.info = !self.info;
                }
            });
        });

        let info = self.note_font_size();
        let width = if self.is_mobile() {
            self.resolution().0 * 0.9
        } else {
            self.resolution().0 / 2.
        };
        egui::Window::new("Info")
            .open(&mut self.info)
            .max_width(width) // FIXME fixed sized window
            .resizable(false)
            .show(ctx, |ui| {
                Self::render_info(ui, info);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Search:");
                        ui.text_edit_singleline(&mut self.search).on_hover_text(
                            "Search for Mission IDs to quickly find the mission you need to do",
                        );
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
                                self.render_mission(
                                    ui,
                                    mission,
                                    #[cfg(not(target_arch = "wasm32"))]
                                    ctx,
                                );
                            }
                        }
                    });
                });

                for (idx, phase) in self.teams.phases.iter().enumerate() {
                    ui.collapsing(
                        egui::RichText::new(format!("Phase {}", idx + 1)).size(
                            self.note_font_size(
                                #[cfg(not(target_arch = "wasm32"))]
                                ctx,
                            ),
                        ),
                        |ui| {
                            self.render_phase(
                                ui,
                                phase,
                                #[cfg(not(target_arch = "wasm32"))]
                                ctx,
                            );
                        },
                    );
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
