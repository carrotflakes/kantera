//#[macro_use]
//extern crate gluten;

use kantera::pixel::Rgba;
use kantera::render::Render;
use kantera::export::render_to_mp4;
use kantera::script::*;

fn main() {
    let res = eval(&make_env(), parse(
        "(plain (rgba (parse_f64 '1.0) (parse_f64 '0.0) (parse_f64 '1.0) (parse_f64 '1.0)))"
    ).unwrap());
    let res = res.borrow();
    let render = std::ops::Deref::deref(res.downcast_ref::<Box<dyn Render<Rgba>>>().unwrap());
    render_to_mp4(
        5.0, 320, 240, 30, 1,
        "script.mp4",
        render);

    println!("done!");
}
