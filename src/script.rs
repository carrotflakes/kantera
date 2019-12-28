use std::rc::Rc;
pub use gluten::{
    data::*,
    parse::parse,
    core::{eval, Env}
};
use crate::{
    image::Image,
    pixel::Rgba,
    render::Render
};

fn parse_f64(s: &String) -> f64 {
    s.parse().unwrap()
}

pub fn make_env() -> Env {
    let mut env = Env::new();
    env.insert("true".to_string(), r(true));
    env.insert("false".to_string(), r(false));
    env.insert("first".to_string(), r(Box::new(|vec: Vec<Val>| {
        vec[0].clone()
    }) as MyFn));
    env.insert("vec".to_string(), r(Box::new(|vec: Vec<Val>| {
        r(vec)
    }) as MyFn));
    env.insert("parse_f64".to_string(), fun!(parse_f64(&String)));
    env.insert("add_f64".to_string(), r(Box::new(|vec: Vec<Val>| -> Val {
        let mut acc = 0.0;
        for rv in vec {
            acc += *rv.borrow().downcast_ref::<f64>().unwrap();
        }
        r(acc)
    }) as MyFn));
    env.insert("sub_f64".to_string(), r(Box::new(|vec: Vec<Val>| -> Val {
        let mut acc = *vec[0].borrow().downcast_ref::<f64>().unwrap();
        for rv in vec.iter().skip(1) {
            acc -= *rv.borrow().downcast_ref::<f64>().unwrap();
        }
        r(acc)
    }) as MyFn));
    env.insert("mul_f64".to_string(), r(Box::new(|vec: Vec<Val>| -> Val {
        let mut acc = 1.0;
        for rv in vec {
            acc *= *rv.borrow().downcast_ref::<f64>().unwrap();
        }
        r(acc)
    }) as MyFn));
    env.insert("div_f64".to_string(), r(Box::new(|vec: Vec<Val>| -> Val {
        let mut acc = *vec[0].borrow().downcast_ref::<f64>().unwrap();
        for rv in vec.iter().skip(1) {
            acc /= *rv.borrow().downcast_ref::<f64>().unwrap();
        }
        r(acc)
    }) as MyFn));
    env.insert("rgba".to_string(), r(Box::new(|vec: Vec<Val>| {
        r(Rgba(
            *vec[0].borrow().downcast_ref::<f64>().unwrap(),
            *vec[1].borrow().downcast_ref::<f64>().unwrap(),
            *vec[2].borrow().downcast_ref::<f64>().unwrap(),
            *vec[3].borrow().downcast_ref::<f64>().unwrap()
        ))
    }) as MyFn));
    env.insert("plain".to_string(), r(Box::new(|vec: Vec<Val>| {
        let p = *vec[0].borrow().downcast_ref::<Rgba>().unwrap();
        r(Some(Rc::new(crate::renders::plain::Plain(p)) as Rc<dyn Render<Rgba>>))
    }) as MyFn));
    env.insert("sequence".to_string(), r(Box::new(|vec: Vec<Val>| {
        let mut sequence = crate::renders::sequence::Sequence::new();
        for p in vec.into_iter() {
            let p = p.borrow().downcast_ref::<Vec<Val>>().unwrap().clone();
            let time = *p[0].borrow().downcast_ref::<f64>().unwrap();
            let restart = *p[1].borrow().downcast_ref::<bool>().unwrap();
            let render = p[2].borrow_mut().downcast_mut::<Option<Rc<dyn Render<Rgba>>>>().unwrap().take().unwrap();
            sequence = sequence.append(time, restart, render);
        }
        r(Some(Rc::new(sequence) as Rc<dyn Render<Rgba>>))
    }) as MyFn));
    env.insert("image_render".to_string(), r(Box::new(|vec: Vec<Val>| {
        let image = vec[0].borrow().downcast_ref::<Rc<Image<Rgba>>>().unwrap().clone();
        r(Some(Rc::new(crate::renders::image_render::ImageRender {
            image: image,
            sizing: crate::renders::image_render::Sizing::Contain
        }) as Rc<dyn Render<Rgba>>))
    }) as MyFn));
    env.insert("text_to_image".to_string(), r(Box::new(|vec: Vec<Val>| {
        let string = vec[0].borrow().downcast_ref::<String>().unwrap().clone();
        use crate::{text::{Font, render}};
        let font_path = "../IPAexfont00401/ipaexg.ttf"; // TODO
        let bytes = std::fs::read(font_path).unwrap();
        let font = Font::from_bytes(&bytes).unwrap();
        // TODO: font size
        r(Rc::new(render(&font, &string)))
    }) as MyFn));
    env.insert("composite".to_string(), r(Box::new(|vec: Vec<Val>| {
        use crate::{renders::composite::{Composite, CompositeMode}, path::Path};
        let layers = vec.into_iter().map(|p| {
            let p = p.borrow().downcast_ref::<Vec<Val>>().unwrap().clone();
            let render = p[0].borrow_mut().downcast_mut::<Option<Rc<dyn Render<Rgba>>>>().unwrap().take().unwrap();
            let mode = p[1].borrow().downcast_ref::<String>().unwrap().to_owned();
            let mode = match mode.as_str() {
                "none" => CompositeMode::None,
                "normal" => CompositeMode::Normal(Path::new(1.0)),
                _ => panic!("illegal CompositeMode")
            };
            (render, mode)
        }).collect();
        r(Some(Rc::new(Composite {
            layers: layers
        }) as Rc<dyn Render<Rgba>>))
    }) as MyFn));
    env
}
