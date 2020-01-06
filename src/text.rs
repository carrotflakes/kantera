extern crate rusttype;

pub use rusttype::*;
use crate::image::Image;

pub fn render(font: &Font, scale: f32, text: &str) -> Image<f64> {
    let scale = Scale::uniform(scale);
    let v_metrics = font.v_metrics(scale);

    let glyphs: Vec<_> = font.layout(text, scale, point(20.0, 20.0 + v_metrics.ascent)).collect();

    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    let (width, height) = (glyphs_width as usize + 40, glyphs_height as usize + 40);
    let mut vec = vec![0.0; width * height];

    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as usize + bounding_box.min.x as usize;
                let y = y as usize + bounding_box.min.y as usize;
                let v = v.min(1.0); // MEMO: v could exceeds 1.0
                vec[x + width * y] = v as f64;
            });
        }
    }

    Image {
        width,
        height,
        vec
    }
}
