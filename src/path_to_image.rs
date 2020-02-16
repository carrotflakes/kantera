use crate::v::Vec2;
use crate::path::{Path, Point};
use crate::pixel::Rgba;
use crate::image::Image;
use crate::cairo::{render_image, Context};

pub fn closed_path_to_image(rect: (i32, i32, i32, i32), stroke_color: Rgba, fill_color: Rgba, line_width: f64, path: &Path<Vec2<f64>>) -> Image<Rgba> {
    let builder = |ctx: Context| {
        ctx.set_line_width(line_width);
        ctx.translate(-rect.0 as f64, -rect.1 as f64);
        ctx.move_to((path.points[0].1).0, (path.points[0].1).1);
        for pair in path.points.windows(2) {
            let prev = pair[0];
            let p = pair[1];
            match p.2 {
                Point::Constant => ctx.line_to((p.1).0, (p.1).1), // ?
                Point::Linear => ctx.line_to((p.1).0, (p.1).1),
                Point::Bezier(lh, rh) => ctx.curve_to((prev.1).0 + lh.0, (prev.1).1 + lh.1, (p.1).0 + rh.0, (p.1).1 + rh.1, (p.1).0, (p.1).1)
            }
        }
        ctx.close_path();
        ctx.set_source_rgba(fill_color.0, fill_color.1, fill_color.2, fill_color.3);
        ctx.fill_preserve();
        ctx.set_source_rgba(stroke_color.0, stroke_color.1, stroke_color.2, stroke_color.3);
        ctx.stroke();
    };
    render_image((rect.2 - rect.0) as usize, (rect.3 - rect.1) as usize, &builder)
}
