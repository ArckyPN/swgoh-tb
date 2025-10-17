use std::path;

use resvg::{tiny_skia, usvg};
use svg::node::element::Polygon;

pub const UNIT: f32 = 1024.;
pub const OUTPUT: &str = "../../assets";
pub const STROKE: &str = "#000000";
pub const STROKE_WIDTH: &str = "5";
pub const GREEN: &str = "#00ff00";
pub const WHITE: &str = "#ffffff";
pub const RED: &str = "#ff0000";

/// replaces `f32`s in `s` with the result of
/// `UNIT * f32`
pub fn path(s: &str) -> String {
    s.split(' ')
        .map(|x| match x.parse::<f32>() {
            Ok(f) => (UNIT * f).to_string(),
            Err(_) => x.to_owned(),
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn points(pts: &[(f32, f32)]) -> String {
    pts.iter()
        .map(|(x, y)| format!("{},{}", UNIT * x, UNIT * y))
        .collect::<Vec<_>>()
        .join(" ")
}

/// saves `svg` at path to PNG with a specified `size`
/// (squared) and writes them into `dir`
///
/// adapted from [resvg::example](https://github.com/linebender/resvg/blob/main/crates/resvg/examples/minimal.rs)
pub fn save_png<P, P2>(svg: P, size: u32, dir: P2, file_name: Option<&str>)
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
    let path = dir.as_ref().join(if let Some(file_name) = file_name {
        file_name.to_owned()
    } else {
        format!("icon-{size}.png")
    });
    pixmap.save_png(path).expect("failed to save png");
}

/// builds the un-rotated cross using a linewidth
///
/// ```
///      P4      P5
///       X-----X
///       |     |
/// P2    |     |      P7
/// X-----X     X-----X
/// |      P3    R6   |
/// |                 |
/// X-----X     X-----X
/// P1    |P12  |P9    P8
///       |     |
///       X-----X
///        P11   P10
/// ```
fn polygon(linewidth: f32, padding: f32) -> [(f32, f32); 12] {
    let min = padding;
    let max = 1. - padding;
    const MID: f32 = 0.5;
    [
        (min, MID + linewidth / 2.),                  // P1
        (min, MID - linewidth / 2.),                  // P2
        (MID - linewidth / 2., MID - linewidth / 2.), // P3
        (MID - linewidth / 2., min),                  // P4
        (MID + linewidth / 2., min),                  // P5
        (MID + linewidth / 2., MID - linewidth / 2.), // P6
        (max, MID - linewidth / 2.),                  // P7
        (max, MID + linewidth / 2.),                  // P8
        (MID + linewidth / 2., MID + linewidth / 2.), // P9
        (MID + linewidth / 2., max),                  // P10
        (MID - linewidth / 2., max),                  // P11
        (MID - linewidth / 2., MID + linewidth / 2.), // P12
    ]
}

pub fn cross(linewidth: f32, fill: &str, padding: f32) -> Polygon {
    Polygon::new()
        .set("points", points(&polygon(linewidth, padding)))
        .set("stroke", STROKE)
        .set("fill", fill)
        .set("stroke-width", STROKE_WIDTH)
        .set(
            "transform",
            format!("rotate(45, {})", points(&[(0.5, 0.5)])),
        )
}
