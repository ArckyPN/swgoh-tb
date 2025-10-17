use std::path::PathBuf;

use svg::{
    Document,
    node::element::{Path, Text},
};

use crate::help::helper;
use icon_generator::{OUTPUT, STROKE, STROKE_WIDTH, UNIT, WHITE, path, save_png};

pub fn build() {
    let svg = helper(Document::new().set("viewBox", (0, 0, UNIT, UNIT)), false)
        .add(silhouette())
        .add(question_mark());

    svg::save("temp.svg", &svg).expect("failed to save svg");

    let output = PathBuf::new().join(OUTPUT).join("img");
    save_png("temp.svg", 512, &output, Some("icon-placeholder.png"));

    std::fs::remove_file("temp.svg").expect("failed to clean workspace");
}

fn silhouette() -> Path {
    Path::new()
        .set("stroke", STROKE)
        .set("stroke-width", STROKE_WIDTH)
        .set("fill", WHITE)
        .set(
            "d",
            path(
                "\
                M 0.0 0.9 \
                Q 0.0 0.8 0.2 0.8 \
                Q 0.4 0.8 0.4 0.7 \
                h -0.2 \
                Q 0.2 0.4 0.3 0.3 \
                Q 0.3 0.1 0.5 0.1 \
                Q 0.7 0.1 0.7 0.3 \
                Q 0.8 0.4 0.8 0.7 \
                h -0.2 \
                Q 0.6 0.8 0.8 0.8 \
                Q 1.0 0.8 1.0 0.9 \
                L 1.0 1.0 h -1.0 v -0.2 \
                z",
            ),
        )
}

fn question_mark() -> Text {
    Text::new("?")
        .set("x", path("0.5"))
        .set("y", path("0.66"))
        .set("font-size", path("0.5"))
        .set("text-anchor", "middle")
}
