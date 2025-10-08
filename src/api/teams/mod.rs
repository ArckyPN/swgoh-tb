mod mission;
mod phase;
mod planet;

pub use mission::*;
pub use phase::*;
pub use planet::*;

macro_rules! phases {
    ( $($num:literal),+ ) => {
        vec![$(
            toml::from_slice(include_bytes!(concat!("../../../assets/data/Phase", $num, ".toml")))
                .expect(&format!("failed to load phase {}", $num)),
        )+]
    };
}

#[derive(Debug)]
pub struct Teams {
    pub phases: Vec<Phase>,
}

impl Teams {
    pub fn load() -> Self {
        Self {
            phases: phases!(1, 2, 3, 4, 5, 6),
        }
    }

    pub fn search(&self, s: &str) -> Vec<Mission> {
        let mut found = Vec::new();

        for phase in &self.phases {
            for mission in &phase.dark.missions {
                if mission
                    .id
                    .to_ascii_lowercase()
                    .starts_with(&s.to_ascii_lowercase())
                {
                    found.push(mission.clone());
                }
            }
            for mission in &phase.mixed.missions {
                if mission
                    .id
                    .to_ascii_lowercase()
                    .starts_with(&s.to_ascii_lowercase())
                {
                    found.push(mission.clone());
                }
            }
            for mission in &phase.light.missions {
                if mission
                    .id
                    .to_ascii_lowercase()
                    .starts_with(&s.to_ascii_lowercase())
                {
                    found.push(mission.clone());
                }
            }
        }

        found
    }
}
