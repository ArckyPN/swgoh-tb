use crate::Mission;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Planet {
    pub name: String,
    pub notes: Option<Vec<String>>,
    #[serde(rename = "mission")]
    pub missions: Vec<Mission>,
}

impl Planet {
    pub fn is_mandalore(&self) -> bool {
        self.name.eq_ignore_ascii_case("Mandalore")
    }

    pub fn is_zeffo(&self) -> bool {
        self.name.eq_ignore_ascii_case("Zeffo")
    }
}
