use crate::Planet;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Phase {
    #[serde(rename = "Dark")]
    pub dark: Planet,
    #[serde(rename = "Mixed")]
    pub mixed: Planet,
    #[serde(rename = "Light")]
    pub light: Planet,
    #[serde(rename = "Bonus")]
    pub bonus: Option<Planet>,
}
