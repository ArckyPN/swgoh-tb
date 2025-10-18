use crate::{BASE_OUTPUT, BASE_URL, Type};
use swgoh_tb::{Omicron, Unit, Units};

use std::path::PathBuf;

use anyhow::{Context as _, Result, bail};
use thirtyfour::{extensions::addons::firefox::FirefoxTools, prelude::*};

#[derive(Debug)]
pub struct Crawler {
    driver: WebDriver,
    units: Vec<Unit>,
    units_toml: PathBuf,
}

impl Crawler {
    pub async fn new() -> Result<Self> {
        let units_toml = PathBuf::new()
            .join(BASE_OUTPUT)
            .join("data")
            .join("Units.toml");
        let units = if std::fs::exists(&units_toml)? {
            let buf = std::fs::read(&units_toml)?;
            let temp: Units = toml::from_slice(&buf)?;
            temp.data
        } else {
            vec![]
        };

        log::debug!("connecting to gecko driver");
        let caps = DesiredCapabilities::firefox();
        let driver = WebDriver::new("http://localhost:4444", caps).await?;

        Ok(Self {
            units,
            driver,
            units_toml,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        self.install_addons().await?;
        log::debug!("beginning crawl");
        self.units().await?;
        self.save()?;

        Ok(())
    }

    pub async fn quit(self) -> Result<()> {
        self.driver.quit().await?;
        Ok(())
    }

    async fn install_addons(&self) -> Result<()> {
        let addons = Vec::from([
            (
                "no-cookies",
                "https://addons.mozilla.org/en-US/firefox/addon/istilldontcareaboutcookies/",
            ),
            (
                "ad-block",
                "https://addons.mozilla.org/en-US/firefox/addon/ublock-origin/",
            ),
        ]);

        for (k, v) in addons {
            let tools = FirefoxTools::new(self.driver.handle.clone());

            // fetch the latest version of each addon
            self.driver.goto(v).await?;
            log::debug!("loaded {}", self.driver.current_url().await?);

            // get the download url of the addon
            let url = self
                .driver
                .query(By::ClassName("AMInstallButton-button"))
                .and_clickable()
                .first()
                .await?
                .attr("href")
                .await?
                .context("missing add button")?;
            let file_name = url.split('/').next_back().context("missing file name")?;

            let path = std::env::current_dir()?.join(file_name);
            // download it if wasn't before / a new version is released
            if !std::fs::exists(&path)? {
                log::debug!("downloading {k} addon...");

                let res = reqwest::get(url).await?;
                let data = res.bytes().await?;
                std::fs::write(&path, data)?;
            }

            // install addon
            tools
                .install_addon(path.to_str().context("unreachable")?, None)
                .await?;
            log::info!("successfully install {k} addon");
        }

        Ok(())
    }

    async fn units(&mut self) -> Result<()> {
        // collect character and ship URLs
        for typ in [Type::Character, Type::Ship] {
            let mut uris = Vec::new();

            self.driver.goto(format!("{BASE_URL}/{typ}/")).await?;
            log::debug!("loaded {}", self.driver.current_url().await?);

            log::debug!("attempting to load {typ}");
            let unit_cards = self
                .driver
                .query(By::Tag("a"))
                .or(By::ClassName("js-unit-search__auto-click"))
                .or(By::ClassName("link-no-style"))
                .and_clickable()
                .any()
                .await?;
            log::debug!("found {} {typ} cards", unit_cards.len());

            for unit_card in unit_cards {
                match unit_card.attr("href").await? {
                    Some(s) if s.starts_with("/units/") => {
                        log::debug!("found valid unit: {s}");
                        uris.push(s);
                    }
                    x => {
                        log::debug!("skipping {x:?} character card");
                    }
                }
            }

            for s in uris {
                self.process_unit(&s, typ).await?;
            }
        }

        Ok(())
    }

    async fn process_unit(&mut self, uri: &str, typ: Type) -> Result<()> {
        self.driver.goto(format!("{BASE_URL}{uri}")).await?;
        log::debug!("loaded {}", self.driver.current_url().await?);

        // get character name
        let name = self
            .driver
            .query(By::Tag("h1"))
            .or(By::ClassName("m-0"))
            .first()
            .await?
            .inner_html()
            .await?;
        log::info!("found unit {name}");
        let name = crate::clean_name(&name);

        if self.has_unit(&name) {
            log::debug!("skipping {name} because it already exists");
            return Ok(());
        }

        // get image
        let elem = self
            .driver
            .query(By::ClassName(format!("{}-portrait__img", typ.portrait())))
            .or(By::Tag("img"))
            .first()
            .await?;
        let url = elem.attr("src").await?.context("missing image href")?;
        log::debug!("found image url of {name}: {url}");

        let filename = url.split('/').next_back().context("missing file name")?;

        // download image
        log::debug!("downloading image");
        let res = reqwest::get(&url).await?;
        let data = res.bytes().await?;

        let path = PathBuf::new().join(BASE_OUTPUT).join("img").join(filename);
        std::fs::write(&path, data)?;
        log::debug!("saved image to {path:?}");

        let unit = Unit::new(&name, filename);

        self.units.push(unit);

        if let Type::Character = typ {
            self.omicron(&name).await?;
        }

        Ok(())
    }

    async fn omicron(&mut self, unit: &str) -> Result<()> {
        let container = self.driver.query(By::ClassName("col-lg-8")).first().await?;

        let mut unit_opt = None;
        for u in self.units.iter_mut() {
            if u.name == unit {
                unit_opt = Some(u);
            }
        }
        let unit = unit_opt.expect("impossible");

        let mut special = 0;
        let mut unique = 0;

        for ability in container.query(By::ClassName("paper")).any().await? {
            let ability_type =
                crate::class_inner_html(&ability, "unit-ability__ability-level").await?;
            let ability_type = crate::enclosed_substring(&ability_type, "(", ")");

            if ability_type.eq_ignore_ascii_case("special") {
                special += 1;
            }
            if ability_type.eq_ignore_ascii_case("unique") {
                unique += 1;
            }

            // skip if not a TB omicron ability
            if !crate::class_inner_html(&ability, "unit-ability__description")
                .await?
                .contains("Territory Battle")
            {
                continue;
            }

            let name = crate::class_inner_html(&ability, "text-white").await?;
            let name = crate::clean_name(&name);

            let ability_type = match ability_type.as_str() {
                "Basic" => Omicron::Basic,
                "Leader" => Omicron::Lead,
                "Special" => Omicron::Special(special),
                "Unique" => Omicron::Unique(unique),
                x => bail!("invalid ability type: {x}"),
            };

            unit.insert_omicron(&ability_type.to_id(), &name);
        }

        Ok(())
    }

    fn save(&self) -> Result<()> {
        let mut units = Units {
            data: self.units.clone(),
        };
        units.data.sort_by(|a, b| a.name.cmp(&b.name));
        let s = toml::to_string(&units)?;

        std::fs::write(&self.units_toml, s)?;
        Ok(())
    }

    fn has_unit(&self, name: &str) -> bool {
        for unit in &self.units {
            if unit.name == name {
                return true;
            }
        }
        false
    }
}
