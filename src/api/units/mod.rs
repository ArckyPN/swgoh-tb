mod unit;

pub use unit::Unit;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Units {
    #[serde(rename = "Unit")]
    pub data: Vec<Unit>,
}

impl Units {
    pub fn load() -> Self {
        toml::from_slice(include_bytes!("../../../assets/data/Units.toml"))
            .expect("failed to load units")
    }

    pub fn get(&self, id: &str) -> Unit {
        if id.is_empty() {
            return Unit::unavailable();
        }
        if id.eq_ignore_ascii_case("[ph]") {
            return Unit::placeholder();
        }
        for unit in &self.data {
            if unit.id.eq_ignore_ascii_case(id) {
                return unit.clone();
            }
        }
        Unit::missing()
    }
}
