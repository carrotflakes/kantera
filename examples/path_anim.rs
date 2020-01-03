use std::rc::Rc;
use kantera::{
    pixel::Rgba,
    v::Vec2,
    path::{Path, Point, Timed},
    render::Render,
    renders::{
        image_render::{ImageRender, Sizing},
        composite::{Composite, CompositeMode},
    },
    export::render_to_mp4,
    renders::functional_render::FunctionalRender,
};

fn main() {
    let path = Path::new(Vec2(50.0, 100.0))
        .append(1.0, Vec2(50.0, 50.0), Point::Linear)
        .append(1.0, Vec2(100.0, 50.0), Point::Bezier(Vec2(0.0, 20.0), Vec2(0.0, -20.0)))
        .append(1.0, Vec2(150.0, 50.0), Point::Bezier(Vec2(-15.0, -15.0), Vec2(15.0, 15.0)))
        .append(1.0, Vec2(150.0, 100.0), Point::Bezier(Vec2(60.0, 0.0), Vec2(0.0, 0.0)))
        .append(1.0, Vec2(50.0, 150.0), Point::Constant)
        .append(1.0, Vec2(75.0, 150.0), Point::Bezier(Vec2(0.0, -40.0), Vec2(0.0, -40.0)))
        .append(1.0, Vec2(100.0, 150.0), Point::Bezier(Vec2(0.0, -40.0), Vec2(0.0, -40.0)))
        .append(1.0, Vec2(125.0, 150.0), Point::Bezier(Vec2(0.0, -40.0), Vec2(0.0, -80.0)))
        .append(1.0, Vec2(130.0, 150.0), Point::Bezier(Vec2(0.0, -80.0), Vec2(0.0, -40.0)));

    let image = Rc::new(kantera::cairo::render_image(320, 240, &|ctx| {
        for w in path.points.windows(2) {
            let (left, right) = (w[0], w[1]);
            ctx.move_to((left.1).0, (left.1).1);
            ctx.line_to((right.1).0, (right.1).1);
            match right.2 {
                Point::Constant => ctx.set_source_rgb(0.6, 0.2, 0.2),
                Point::Linear => ctx.set_source_rgb(0.2, 0.6, 0.2),
                Point::Bezier(_, _) => ctx.set_source_rgb(0.2, 0.2, 0.6)
            }
            ctx.stroke();
        }
        for p in path.points.iter() {
            match p.2 {
                Point::Bezier(lh, rh) => {
                    ctx.move_to((p.1).0 + lh.0, (p.1).1 + lh.1);
                    ctx.line_to((p.1).0, (p.1).1);
                    ctx.line_to((p.1).0 + rh.0, (p.1).1 + rh.1);
                    ctx.set_source_rgb(0.4, 0.4, 0.4);
                    ctx.stroke();
                },
                _ => {}
            }
            ctx.arc((p.1).0, (p.1).1, 2.0, 0.0, std::f64::consts::PI * 2.0);
            ctx.set_source_rgb(0.8, 0.8, 0.8);
            ctx.fill();
        }
        for i in 0..100 {
            let p = path.get_value(i as f64 / 10.0);
            ctx.arc(p.0, p.1, 1.0, 0.0, std::f64::consts::PI * 2.0);
            ctx.set_source_rgba(1.0, 1.0, 1.0, 0.25);
            ctx.fill();
        }
    }));

    render_to_mp4(
        10.0, 320, 240, 30, 1,
        "path_anim.mp4",
        &Composite::<Box<dyn Render<Rgba>>> {
            layers: vec![
                (
                    Box::new(ImageRender {
                        image: image.clone(), sizing: Sizing::Fit, default: Rgba(0.0, 0.0, 0.0, 0.0),
                        interpolation: kantera::interpolation::Bilinear
                    }),
                    CompositeMode::None
                ),
                (
                    Box::new(FunctionalRender(Box::new(move |ro, time, buffer| {
                        let w = ro.u_res;
                        let h = ro.v_res;
                        for y in 0..h {
                            for x in 0..w {
                                let p = path.get_value(time);
                                let d = (p.0 - x as f64).hypot(p.1 - y as f64);
                                let v = (3.0 - d).min(1.0).max(0.0);
                                buffer[y * w + x] = Rgba(v, 0.0, 0.0, v);
                            }
                        }
                    }))),
                    CompositeMode::Normal(Path::new(1.0))
                )
            ]
    });

    println!("done!");
}
