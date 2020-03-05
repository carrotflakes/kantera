use crate::v::Vec2;
use crate::path::{Path, Point};
use crate::pixel::Rgba;
use crate::image::Image;
use crate::cairo::{render_image, Context};

pub fn closed_path_to_image(rect: [i32; 4], stroke_color: Rgba, fill_color: Rgba, line_width: f64, path: &Path<Vec2<f64>>) -> Image<Rgba> {
    let builder = |ctx: Context| {
        ctx.set_line_width(line_width);
        ctx.translate(-rect[0] as f64, -rect[1] as f64);
        ctx.move_to((path.points[0].1).0, (path.points[0].1).1);
        for pair in path.points.windows(2) {
            let p = pair[1];
            match p.2 {
                Point::Constant => ctx.line_to((p.1).0, (p.1).1), // ?
                Point::Linear => ctx.line_to((p.1).0, (p.1).1),
                Point::Bezier2(_) => panic!("closed_path_to_image not support Bezier2"),
                Point::Bezier3(h1, h2) => ctx.curve_to(h1.0, h1.1, h2.0, h2.1, (p.1).0, (p.1).1)
            }
        }
        ctx.close_path();
        ctx.set_source_rgba(fill_color.0, fill_color.1, fill_color.2, fill_color.3);
        ctx.fill_preserve();
        ctx.set_source_rgba(stroke_color.0, stroke_color.1, stroke_color.2, stroke_color.3);
        ctx.stroke();
    };
    render_image((rect[2] - rect[0]) as usize, (rect[3] - rect[1]) as usize, &builder)
}

pub fn closed_path_rect(path: &Path<Vec2<f64>>) -> [i32; 4] {
    let mut rect = [0f64, 0.0, 0.0, 0.0];
    rect[0] = (path.points[0].1).0;
    rect[1] = (path.points[0].1).1;
    rect[2] = (path.points[0].1).0;
    rect[3] = (path.points[0].1).1;
    for pair in path.points.windows(2) {
        let prev = pair[0];
        let p = pair[1];
        match p.2 {
            Point::Constant => { // ?
                rect[0] = rect[0].min((p.1).0);
                rect[1] = rect[1].min((p.1).1);
                rect[2] = rect[2].max((p.1).0);
                rect[3] = rect[3].max((p.1).1);
            },
            Point::Linear => {
                rect[0] = rect[0].min((p.1).0);
                rect[1] = rect[1].min((p.1).1);
                rect[2] = rect[2].max((p.1).0);
                rect[3] = rect[3].max((p.1).1);
            },
            Point::Bezier2(_) => {
                panic!("closed_path_to_image not support Bezier2");
            },
            Point::Bezier3(h1, h2) => {
                rect[0] = rect[0].min((prev.1).0).min(h1.0).min(h2.0).min((p.1).0);
                rect[1] = rect[1].min((prev.1).1).min(h1.1).min(h2.1).min((p.1).1);
                rect[2] = rect[2].max((prev.1).0).max(h1.0).max(h2.0).max((p.1).0);
                rect[3] = rect[3].max((prev.1).1).max(h1.1).max(h2.1).max((p.1).1);
            }
        }
    }
    [rect[0].floor() as i32, rect[1].floor() as i32, rect[2].ceil() as i32, rect[3].ceil() as i32]
}

pub fn expand_rect(rect: [i32; 4], size: i32) -> [i32; 4] {
    [rect[0] - size, rect[1] - size, rect[2] + size, rect[3] + size]
}
