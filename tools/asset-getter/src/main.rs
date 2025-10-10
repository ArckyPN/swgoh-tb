use std::{fmt::Display, path::PathBuf};

use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};
use thirtyfour::{extensions::addons::firefox::FirefoxTools, prelude::*};

const BASE_URL: &str = "https://swgoh.gg";
const BASE_OUTPUT: &str = "../../assets";

#[derive(Debug, Copy, Clone)]
enum Type {
    Character,
    Ship,
}

impl Type {
    fn portrait(&self) -> String {
        match self {
            Self::Character => "character".to_owned(),
            Self::Ship => "ship".to_owned(),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Character => f.write_str("characters"),
            Self::Ship => f.write_str("ships"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Unit {
    pub id: String,
    pub name: String,
    pub image: String,
}

impl Unit {
    pub fn new(name: &str, image: &str) -> Self {
        Self {
            id: "# TODO: add id".to_owned(),
            name: name.to_owned(),
            image: image.to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Units {
    #[serde(rename = "Unit")]
    data: Vec<Unit>,
}

#[derive(Debug)]
struct UnitsCrawler {
    units: Vec<Unit>,
    driver: WebDriver,
    units_toml: PathBuf,
}

impl UnitsCrawler {
    async fn new() -> Result<Self> {
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
            let file_name = url.split('/').last().context("missing file name")?;

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

    async fn run(&mut self) -> Result<()> {
        self.install_addons().await?;
        log::debug!("beginning crawl");
        self.units().await?;
        self.save()?;
        Ok(())
    }

    async fn quit(self) -> Result<()> {
        self.driver.quit().await?;
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
                        continue;
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

        let filename = url.split('/').last().context("missing file name")?;

        // download image
        log::debug!("downloading image");
        let res = reqwest::get(&url).await?;
        let data = res.bytes().await?;

        let path = PathBuf::new().join(BASE_OUTPUT).join("img").join(filename);
        std::fs::write(&path, data)?;
        log::debug!("saved image to {path:?}");

        let unit = Unit::new(&name, filename);

        self.units.push(unit);

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

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let mut crawler = UnitsCrawler::new().await?;
    if let Err(err) = crawler.run().await {
        log::error!("{err}");
    }
    crawler.quit().await
}
