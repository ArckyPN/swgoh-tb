use std::path::PathBuf;

use svg::Document;

use crate::help::helper;
use icon_generator::{OUTPUT, RED, UNIT, cross, save_png};

pub fn build() {
    let svg = helper(Document::new().set("viewBox", (0, 0, UNIT, UNIT)), false)
        .add(cross(0.2, RED, -0.25));

    svg::save("temp.svg", &svg).expect("failed to save svg");

    let output = PathBuf::new().join(OUTPUT).join("img");
    save_png("temp.svg", 512, &output, Some("icon-missing.png"));

    std::fs::remove_file("temp.svg").expect("failed to clean workspace");
}
