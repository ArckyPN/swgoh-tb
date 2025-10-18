use serde::{Deserialize, Serialize};

use crate::Ability;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Unit {
    pub id: String,
    pub name: String,
    pub image: String,
    pub omicrons: Vec<Ability>,
}

impl Unit {
    pub fn new(name: &str, image: &str) -> Self {
        Self {
            // ugly to prevent this from being registered as an actual todo
            id: format!("# {}: add id", "todo".to_ascii_uppercase()),
            name: name.to_owned(),
            image: image.to_owned(),
            omicrons: vec![],
        }
    }

    pub fn insert_omicron(&mut self, id: &str, name: &str) {
        if self.has_omicron(id) {
            return;
        }
        self.omicrons.push(Ability {
            id: id.to_owned(),
            name: name.to_owned(),
        });
    }

    pub fn has_omicron(&self, id: &str) -> bool {
        for omi in &self.omicrons {
            if omi.id == id {
                return true;
            }
        }
        false
    }

    pub fn get_omicron(&self, id: &str) -> Ability {
        for omi in &self.omicrons {
            if omi.id == id {
                return omi.clone();
            }
        }
        Ability {
            id: "404".to_owned(),
            name: "Unknown Ability".to_owned(),
        }
    }

    pub fn image(&self, origin: &str) -> String {
        format!("{origin}/assets/img/{}", self.image)
    }

    pub fn missing() -> Self {
        Self {
            id: String::new(),
            name: "Unknown Unit".to_owned(),
            image: "icon-missing.png".to_owned(),
            omicrons: vec![],
        }
    }

    pub fn placeholder() -> Self {
        Self {
            id: "[ph]".to_owned(),
            name: "open spot".to_owned(),
            image: "icon-placeholder.png".to_owned(),
            omicrons: vec![],
        }
    }

    pub fn unavailable() -> Self {
        Self {
            id: "unavailable".to_owned(),
            name: "unavailable".to_owned(),
            image: "icon-unavailable.png".to_owned(),
            omicrons: vec![],
        }
    }

    pub fn forgot() -> Self {
        Self {
            id: String::new(),
            name: "forgot to add all spots in PhaseX.toml".to_owned(),
            image: "icon-missing.png".to_owned(),
            omicrons: vec![],
        }
    }
}
