use svg::node::element::{Group, Line, Rectangle};

use crate::{FILL_T, STROKE, path};

pub fn helper(enable: bool) -> Option<Group> {
    if !enable {
        return None;
    }

    Some(
        Group::new()
            .add(
                // white background
                Rectangle::new()
                    .set("x", "0")
                    .set("y", "0")
                    .set("width", path("1"))
                    .set("height", path("1"))
                    .set("stroke", STROKE)
                    .set("stroke-width", "1")
                    .set("fill", FILL_T),
            )
            .add(
                // icon frame
                Rectangle::new()
                    .set("x", path("0.2"))
                    .set("y", path("0.2"))
                    .set("width", path("0.6"))
                    .set("height", path("0.6"))
                    .set("stroke", STROKE)
                    .set("stroke-width", "3")
                    .set("fill", FILL_T),
            )
            .add(grid(0.1)),
    )
}

fn grid(interval: f32) -> Group {
    let mut i = interval;
    let mut lines = Vec::new();
    loop {
        lines.push(line(((0., i), (1., i))));
        lines.push(line(((i, 0.), (i, 1.))));
        i += interval;
        println!("{i}");
        if i >= 1. {
            break;
        }
    }
    lines.into_iter().fold(Group::new(), |acc, l| acc.add(l))
}

fn line(pts: ((f32, f32), (f32, f32))) -> Line {
    Line::new()
        .set("x1", path(&pts.0.0.to_string()))
        .set("y1", path(&pts.0.1.to_string()))
        .set("x2", path(&pts.1.0.to_string()))
        .set("y2", path(&pts.1.1.to_string()))
        .set("stroke", "#444444")
        .set("stroke-width", "1")
}
