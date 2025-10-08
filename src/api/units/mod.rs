mod unit;

pub use unit::Unit;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Units {
    #[serde(rename = "Unit")]
    data: Vec<Unit>,
}

impl core::ops::Index<&str> for Units {
    type Output = Unit;
    fn index(&self, index: &str) -> &Self::Output {
        for unit in &self.data {
            if unit.id.eq_ignore_ascii_case(index) {
                return unit;
            }
        }
        &self.data[self.data.len() - 1]
    }
}
