use crate::Mission;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Planet {
    pub name: String,
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

    // pub fn size(&self, len: f32, is_portrait: bool) -> egui::Vec2 {
    //     // TODO
    //     let x= if is_portrait {
    //         if len <
    //      } else { 50. };
    //      let y = 50.
    //     egui::vec2(x, y)
    // }
}
