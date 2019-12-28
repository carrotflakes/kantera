use kantera::pixel::Rgba;
use kantera::render::Render;
use kantera::export::render_to_mp4;
use kantera::script::*;
use std::rc::Rc;

fn main() {
    let res = eval(make_env(), parse(
        //"(plain (rgba (parse_f64 '1.0) (parse_f64 '0.0) (parse_f64 '1.0) (parse_f64 '1.0)))"
        "(sequence
            (vec (parse_f64 '0.0) true (plain (rgba (parse_f64 '1.0) (parse_f64 '0.0) (parse_f64 '0.0) (parse_f64 '1.0))))
            (vec (parse_f64 '1.0) true (plain (rgba (parse_f64 '0.0) (parse_f64 '1.0) (parse_f64 '0.0) (parse_f64 '1.0))))
            (vec (parse_f64 '2.0) true (plain (rgba (parse_f64 '0.0) (parse_f64 '0.0) (parse_f64 '1.0) (parse_f64 '1.0))))
        )"
    ).unwrap());
    let render = res.borrow().downcast_ref::<Option<Rc<dyn Render<Rgba>>>>().unwrap().as_ref().unwrap().clone();
    render_to_mp4(
        5.0, 320, 240, 30, 1,
        "script.mp4",
        &render);

    println!("done!");
}