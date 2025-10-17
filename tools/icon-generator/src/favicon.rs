use std::path::{self, PathBuf};

use svg::{
    Document,
    node::element::{Path, Text},
};
use svg_to_ico::svg_to_ico;

use crate::help::helper;
use icon_generator::{GREEN, OUTPUT, STROKE, STROKE_WIDTH, UNIT, WHITE, path, save_png};

pub fn build() {
    let svg = helper(Document::new().set("viewBox", (0, 0, UNIT, UNIT)), false)
        .add(b())
        .add(t())
        .add(signature());

    svg::save("temp.svg", &svg).expect("failed to save svg");

    let output = PathBuf::new().join(OUTPUT);
    save_png("temp.svg", 1024, &output, None);
    save_png("temp.svg", 512, &output, None);
    save_png("temp.svg", 256, &output, None);
    save_png("temp.svg", 192, &output, None);

    svg_to_ico(
        path::Path::new("temp.svg"),
        96.,
        output.join("favicon.ico").as_path(),
        &[64, 64],
    )
    .expect("unable to convert svg to ico");

    std::fs::remove_file("temp.svg").expect("failed to clean workspace");
}

/// create the svg Path for the B
fn b() -> Path {
    Path::new()
        .set("fill-rule", "evenodd")
        .set("stroke", STROKE)
        .set("stroke-width", STROKE_WIDTH)
        .set("fill", GREEN)
        .set(
            "d",
            path(
                "\
                M 0.6 0.8 \
                h -0.2 v -0.6 h 0.2 \
                q 0.2 0 0.2 0.15 \
                q 0 0.1 -0.1 0.15 \
                q 0.1 0.05 0.1 0.15 \
                q 0 0.15 -0.2 0.15 \
                z \
                M 0.5 0.3 \
                h 0.1 \
                C 0.725 0.3 0.725 0.45 0.6 0.45 \
                h -0.1 v -0.15 \
                m 0 0.25 \
                h 0.1 \
                C 0.725 0.55 0.725 0.7 0.6 0.7 \
                h -0.1 \
                z",
            ),
        )
}

/// create the svg Path for the T
fn t() -> Path {
    Path::new()
        .set("fill-rule", "evenodd")
        .set("stroke", STROKE)
        .set("stroke-width", STROKE_WIDTH)
        .set("fill", WHITE)
        .set(
            "d",
            path(
                "\
                M 0.2 0.2 \
                h 0.4 \
                Q 0.675 0.2 0.7 0.215 \
                L 0.665 0.32 \
                Q 0.65 0.305 0.6 0.3 \
                h -0.1 v 0.05 \
                l -0.1 0.1 \
                v -0.15 h -0.2 v -0.1 \
                z \
                M 0.4 0.5 \
                l 0.1 -0.1 \
                v 0.05 \
                l -0.1 0.1 \
                v -0.05 \
                z \
                M 0.4 0.65 \
                l 0.1 -0.1 \
                v 0.05 \
                l -0.1 0.1 \
                v -0.05 \
                z \
                M 0.4 0.75 \
                l 0.1 -0.1 \
                v 0.15 h -0.1 v -0.1 \
                z",
            ),
        )
}

/// creates the "signature" text
fn signature() -> Text {
    Text::new("Arcky")
        .set("x", path("0.42"))
        .set("y", path("0.785"))
        .set("textLength", path("0.06"))
}
