use crate::Mission;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Planet {
    pub name: String,
    #[serde(rename = "mission")]
    pub missions: Vec<Mission>,
}
