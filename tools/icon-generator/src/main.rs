mod help;

use std::path::{self, PathBuf};

use resvg::{tiny_skia, usvg};
use svg::{
    Document,
    node::element::{Path, Text},
};
use svg_to_ico::svg_to_ico;

use crate::help::helper;

const UNIT: usize = 1024;
const OUTPUT: &str = "../../assets";
const STROKE: &str = "#000000";
const STROKE_WIDTH: &str = "5";
const FILL_B: &str = "#00ff00";
const FILL_T: &str = "#ffffff";

/// replaces `f32`s in `s` with the result of
/// `UNIT * f32`
fn path(s: &str) -> String {
    s.split(' ')
        .map(|x| match x.parse::<f32>() {
            Ok(f) => (UNIT as f32 * f).to_string(),
            Err(_) => x.to_owned(),
        })
        .collect::<Vec<String>>()
        .join(" ")
}

/// create the svg Path for the B
fn b() -> Path {
    Path::new()
        .set("fill-rule", "evenodd")
        .set("stroke", STROKE)
        .set("stroke-width", STROKE_WIDTH)
        .set("fill", FILL_B)
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
        .set("fill", FILL_T)
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

/// saves `svg` at path to PNG with a specified `size`
/// (squared) and writes them into `dir`
///
/// adapted from [resvg::example](https://github.com/linebender/resvg/blob/main/crates/resvg/examples/minimal.rs)
fn save_png<P, P2>(svg: P, size: u32, dir: P2)
where
    P: AsRef<path::Path>,
    P2: AsRef<path::Path>,
{
    let mut opt = usvg::Options::default();
    opt.fontdb_mut().load_system_fonts();

    let buf = std::fs::read(svg).expect("failed to read svg");
    let tree = usvg::Tree::from_data(&buf, &opt).expect("failed to parse svg tree");

    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(size, size).expect("failed to create pixmap");

    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(
            size as f32 / pixmap_size.width() as f32,
            size as f32 / pixmap_size.height() as f32,
        ),
        &mut pixmap.as_mut(),
    );
    let path = dir.as_ref().join(format!("icon-{size}.png"));
    pixmap.save_png(path).expect("failed to save png");
}

fn main() {
    // setting helper to true will draw a grid to help
    // with the design process
    let svg = if let Some(group) = helper(false) {
        Document::new()
            .set("viewBox", (0, 0, UNIT, UNIT))
            .add(group)
            .add(b())
            .add(t())
            .add(signature())
    } else {
        Document::new()
            .set("viewBox", (0, 0, UNIT, UNIT))
            .add(b())
            .add(t())
            .add(signature())
    };

    let output = PathBuf::new().join(OUTPUT);
    svg::save("temp.svg", &svg).expect("failed to save svg");

    save_png("temp.svg", 1024, &output);
    save_png("temp.svg", 512, &output);
    save_png("temp.svg", 256, &output);
    save_png("temp.svg", 192, &output);

    svg_to_ico(
        path::Path::new("temp.svg"),
        96.,
        output.join("favicon.ico").as_path(),
        &[64, 64],
    )
    .expect("unable to convert svg to ico");

    std::fs::remove_file("temp.svg").expect("failed to clean workspace");
}
