pub use gluten::{
    data::*,
    parse::parse,
    core::{eval, Env}
};
use crate::{
    pixel::Rgba,
    render::Render
};

fn parse_f64(s: &String) -> f64 {
    s.parse().unwrap()
}

pub fn make_env() -> Env {
    let mut env = Env::new();
    env.insert("first".to_string(), r(Box::new(|vec: Vec<R<V>>| {
        vec[0].clone()
    }) as MyFn));
    env.insert("rgba".to_string(), r(Box::new(|vec: Vec<R<V>>| {
        r(Rgba(
            *vec[0].borrow().downcast_ref::<f64>().unwrap(),
            *vec[1].borrow().downcast_ref::<f64>().unwrap(),
            *vec[2].borrow().downcast_ref::<f64>().unwrap(),
            *vec[3].borrow().downcast_ref::<f64>().unwrap()
        )) as R<V>
    }) as MyFn));
    env.insert("plain".to_string(), r(Box::new(|vec: Vec<R<V>>| {
        let p = *vec[0].borrow().downcast_ref::<Rgba>().unwrap();
        r(Box::new(crate::renders::plain::Plain(p)) as Box<dyn Render<Rgba>>) as R<V>
    }) as MyFn));
    env.insert("parse_f64".to_string(), fun!(parse_f64(&String)));
    //env.insert("add".to_string(), fun!(add(i32, i32)));
    env
}
