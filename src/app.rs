#![cfg(target_arch = "wasm32")]

use std::str::FromStr as _;

use crate::{Mission, Omicron, Omicrons, Planet, Tab, Teams, Unit, Units, Video};

const CAPITAL_SHIP_FACTOR: f32 = 1.1;
const STARTING_LINEUP_FACTOR: f32 = 0.9;
const REINFORCEMENT_FACTOR: f32 = 0.8;

pub struct App {
    units: Units,
    teams: Teams,
    search: String,
    tab: Tab,

    window: web_sys::Window,
    origin: String,
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, window: web_sys::Window) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        Self {
            units: Units::load(),
            teams: Teams::load(),
            search: Default::default(),
            tab: Tab::from_str(&window.location().hash().unwrap_or("1".to_owned()))
                .unwrap_or_default(),
            window,
            origin: if cc.integration_info.web_info.location.url.contains("dev") {
                cc.integration_info.web_info.location.origin.clone()
            } else {
                cc.integration_info
                    .web_info
                    .location
                    .url
                    .split('#')
                    .next()
                    .expect("split always has first")
                    .to_owned()
            },
        }
    }

    /// screen resolution (width, height) in pixels
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

    fn reference_size(&self) -> f32 {
        let res = self.resolution();
        if self.is_portrait() { res.0 } else { res.1 }
    }

    fn character_icon_size(&self) -> egui::Vec2 {
        let base = self.reference_size();
        let size = base / 20.;
        egui::Vec2::new(size, size)
    }

    fn planet_font_size(&self) -> f32 {
        self.reference_size() / 40.
    }

    fn mission_font_size(&self) -> f32 {
        self.reference_size() / 60.
    }

    fn note_font_size(&self) -> f32 {
        self.reference_size() / 70.
    }

    fn unit_font_size(&self) -> f32 {
        self.reference_size() / 135.
    }

    fn is_portrait(&self) -> bool {
        let res = self.resolution();

        res.1 > res.0
    }

    fn render_phase(&self, ui: &mut egui::Ui, idx: usize) {
        let phase = &self.teams.phases[idx];
        if self.is_portrait() {
            ui.vertical(|ui| {
                for planet in phase {
                    self.render_planet(ui, planet);
                }
            });
        } else {
            ui.columns(phase.num(), |ui| {
                for (col, planet) in phase.iter().enumerate() {
                    self.render_planet(&mut ui[col], planet);
                }
            });
        }
    }

    fn render_planet(&self, ui: &mut egui::Ui, planet: &Planet) {
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new(&planet.name)
                    .strong()
                    .size(self.planet_font_size()),
            );
            for mission in &planet.missions {
                self.render_mission(ui, mission);
            }
        });
    }

    fn render_mission(&self, ui: &mut egui::Ui, mission: &Mission) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new(format!("{} ({})", mission.name, mission.id))
                            .underline()
                            .size(self.mission_font_size()),
                    );
                });

                ui.horizontal(|ui| {
                    if mission.omicrons.is_some() {
                        ui.add_sized(
                            self.character_icon_size() * 0.33,
                            |ui: &mut egui::Ui| -> egui::Response {
                                ui.add(egui::Image::new(format!(
                                    "{}/assets/img/icon-omicron.webp",
                                    self.origin
                                )))
                            },
                        );
                    }

                    ui.label(
                        egui::RichText::new(match &mission.relic {
                            Some(relic) => format!("Relic: {relic}+"),
                            None => "7* Stars".to_owned(),
                        })
                        .size(self.note_font_size()),
                    );
                });

                ui.separator();

                ui.horizontal(|ui| {
                    let missing = if mission.name != "Fleet" {
                        self.render_squad(ui, &mission.team, mission.omicrons.as_deref())
                    } else {
                        self.render_fleet(ui, &mission.team)
                    };
                    self.missing_helper(missing, ui);
                });

                ui.separator();

                self.render_note(ui, &mission.note);

                if let Some(omicrons) = &mission.omicrons {
                    ui.separator();

                    self.render_omicron_list(ui, omicrons);
                }

                if let Some(videos) = &mission.videos {
                    ui.separator();

                    self.render_videos(ui, videos);
                }

                if let Some(modding) = &mission.modding {
                    ui.separator();

                    self.render_modding(ui, modding);
                }
            });
        });
    }

    fn render_note(&self, ui: &mut egui::Ui, notes: &[String]) {
        let size = self.note_font_size();
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("Notes:")
                    .size(size)
                    .underline()
                    .strong(),
            );
            ui.spacing_mut().item_spacing.y = 10.0;
            for note in notes {
                ui.label(egui::RichText::new(note).size(size));
            }
        });
    }

    fn render_omicron_list(&self, ui: &mut egui::Ui, omicrons: &[Omicrons]) {
        let size = self.note_font_size();
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("Required Omicrons:")
                    .size(size)
                    .underline()
                    .strong(),
            );
            for omicron in omicrons {
                let unit = self.units.get(&omicron.unit);
                for omi in &omicron.omis {
                    ui.label(
                        egui::RichText::new(format!(
                            "\t- {}: {} \"{}\"",
                            unit.name, omi, "# TODO ability name"
                        ))
                        .size(size),
                    );
                }
            }
        });
    }

    fn render_videos(&self, ui: &mut egui::Ui, videos: &[Video]) {
        let size = self.note_font_size();
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("Videos:")
                    .size(size)
                    .underline()
                    .strong(),
            );
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                for (idx, video) in videos.iter().enumerate() {
                    if idx != 0 {
                        ui.label(egui::RichText::new(", ").size(size));
                    }
                    ui.hyperlink_to(egui::RichText::new(&video.source).size(size), &video.url);
                }
            });
        });
    }

    fn render_modding(&self, ui: &mut egui::Ui, modding: &str) {
        let size = self.note_font_size();
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("Modding Example: ")
                    .size(size)
                    .underline()
                    .strong(),
            );
            ui.hyperlink_to(egui::RichText::new("Image").size(size), modding);
        });
    }

    /// simple helper to make sure all slots are filled in the PhaseX.toml files
    fn missing_helper(&self, missing: i32, ui: &mut egui::Ui) {
        for _ in 0..missing {
            ui.vertical(|ui| {
                self.render_unit(ui, &Unit::forgot(), self.character_icon_size(), None);
            });
        }
    }

    fn render_unit(
        &self,
        ui: &mut egui::Ui,
        unit: &Unit,
        size: impl Into<egui::Vec2>,
        omicron: Option<&[Omicron]>,
    ) {
        ui.vertical(|ui| {
            ui.add_sized(size, |ui: &mut egui::Ui| -> egui::Response {
                let res = ui.add(egui::Image::new(unit.image(&self.origin)).shrink_to_fit());
                 if unit.id.eq_ignore_ascii_case("[ph]") {
                    res.on_hover_text("open spots can be filled with whatever you want, but generally these spots are not needed");
                } else if unit.id.eq_ignore_ascii_case("unavailable") {
                    res.on_hover_text("this spot cannot be filled");
                } else if let Some(omicron) = omicron {
                    let r#box = egui::Rect { min: egui::Pos2::new((res.rect.max.x + res.rect.min.x) / 2., (res.rect.max.y + res.rect.min.y) / 2.), max: egui::Pos2::new(res.rect.max.x, res.rect.max.y)};
                    ui.place(r#box, egui::Image::new(format!("{}/assets/img/icon-omicron-badge.png", self.origin)));
                    ui.place(r#box, |ui: &mut egui::Ui| -> egui::Response {
                        ui.label(egui::RichText::new(omicron.len().to_string()).strong().size(self.unit_font_size()))
                    });
                }
                
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new(&unit.name).size(self.unit_font_size()));
                })
                .response
            });
        });
    }

    fn render_squad(
        &self,
        ui: &mut egui::Ui,
        team: &[String],
        omicrons: Option<&[Omicrons]>,
    ) -> i32 {
        let mut missing = 5;

        ui.horizontal(|ui| {
            for unit in team {
                missing -= 1;
                let unit = self.units.get(unit);
                let omi = match omicrons {
                    Some(omi) => omi.iter().find_map(|o| {
                        if o.unit.eq_ignore_ascii_case(&unit.id) {
                            return Some(o.omis.clone());
                        }
                        None
                    }),
                    _ => None,
                };
                self.render_unit(ui, &unit, self.character_icon_size(), omi.as_deref());
            }
        });

        missing
    }

    fn render_fleet(&self, ui: &mut egui::Ui, team: &[String]) -> i32 {
        let mut missing = 8;

        let mut team = team.iter().map(|u| self.units.get(u));

        ui.horizontal(|ui| {
            // capital ship
            let cap = team.next().expect("must have capital ship");
            self.render_unit(
                ui,
                &cap,
                self.character_icon_size() * CAPITAL_SHIP_FACTOR,
                None,
            );
            missing -= 1;

            // starting lineup
            for starting in team.by_ref() {
                self.render_unit(
                    ui,
                    &starting,
                    self.character_icon_size() * STARTING_LINEUP_FACTOR,
                    None,
                );
                missing -= 1;
                if missing == 4 {
                    break;
                }
            }

            // reinforcements
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Reinforcements").size(self.unit_font_size()));
                ui.horizontal(|ui| {
                    for reinforcement in team {
                        self.render_unit(
                            ui,
                            &reinforcement,
                            self.character_icon_size() * REINFORCEMENT_FACTOR,
                            None,
                        );
                        missing -= 1;
                    }
                });
            });
        });

        missing
    }

    fn render_info(&self, ui: &mut egui::Ui) {
        let text = |text: &str| egui::RichText::new(text).size(self.note_font_size());

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
            ui.label(text("Where possible, I prefer teams which are able to full auto missions (without Omicrons). When full auto is not possible, I will give alternatives."));
        });
    }

    fn render_search(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                let label = ui.label("Search:");

                ui.add(
                    egui::TextEdit::singleline(&mut self.search)
                        .hint_text("Mission ID")
                        .desired_width(200.)
                        .char_limit(20),
                )
                .labelled_by(label.id)
                .on_hover_text("Search for Mission IDs to quickly find the mission you need to do");
                egui::widgets::reset_button(ui, &mut self.search, "Clear");
            });

            // search results
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
    }

    fn render_navbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            if ui.selectable_label(self.tab == Tab::Info, "Info").clicked() {
                self.tab = Tab::Info;
                self.set_fragment(&self.tab);
            }
            for (idx, _) in self.teams.phases.iter().enumerate() {
                if ui
                    .selectable_label(
                        self.tab == Tab::Phase(idx + 1),
                        format!("Phase {}", idx + 1),
                    )
                    .clicked()
                {
                    self.tab = Tab::Phase(idx + 1);
                    self.set_fragment(&self.tab);
                }
            }
        });
    }

    fn set_fragment(&self, tab: &Tab) {
        self.window
            .location()
            .set_hash(&tab.to_string())
            .expect("failed to set fragment");
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    egui::widgets::global_theme_preference_switch(ui);
                    ui.heading("Rise of the Empire TB Team setup");
                });

                self.render_navbar(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.render_search(ui);

                match self.tab {
                    Tab::Info => self.render_info(ui),
                    Tab::Phase(x) => self.render_phase(ui, x - 1),
                }

                ui.separator();

                ui.vertical(|ui| {
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
        ui.add(egui::github_link_file!(
            "https://github.com/ArckyPN/swgoh-tb/blob/main/",
            "Source code. "
        ));
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(". Made by Arcky (");
        ui.hyperlink_to("swgoh.gg", "https://swgoh.gg/p/121696617/");
        ui.label(", ");
        ui.hyperlink_to("YouTube", "https://www.youtube.com/@Arcky-ykcrA");
        ui.label(", ");
        ui.hyperlink_to(
            "Steam",
            "https://steamcommunity.com/profiles/76561198077815167",
        );
        ui.label(").");
    });
}
