use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Unit {
    pub id: String,
    pub name: String,
    pub image: String,
}

impl Unit {
    pub fn image(&self, origin: &str) -> String {
        format!("{origin}/assets/img/{}", self.image)
    }

    pub fn missing() -> Self {
        Self {
            id: String::new(),
            name: "Unknown Unit".to_owned(),
            image: "missing.png".to_owned(),
        }
    }

    pub fn placeholder() -> Self {
        Self {
            id: "[ph]".to_owned(),
            name: "open spot".to_owned(),
            image: "placeholder.png".to_owned(),
        }
    }

    pub fn unavailable() -> Self {
        Self {
            id: "unavailable".to_owned(),
            name: "unavailable".to_owned(),
            image: "unavailable.png".to_owned(),
        }
    }

    pub fn forgot() -> Self {
        Self {
            id: String::new(),
            name: "forgot to add all spots in PhaseX.toml".to_owned(),
            image: "missing.png".to_owned(),
        }
    }
}
