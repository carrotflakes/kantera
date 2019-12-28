use std::rc::Rc;
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
            //let render = p[2].borrow().downcast_ref::<Box<dyn Render<Rgba>>>().unwrap();
            //let render = std::cell::RefCell::new(() as V);
            //std::mem::replace(p[2].get_mut(), render);
            //let mut rv = p[2].clone();
            //let render = std::mem::replace(std::rc::Rc::make_mut(&mut rv), std::cell::RefCell::new(() as V));
            //let render = unsafe {Box::from_raw(p[2].as_ptr() as *mut Box<dyn Render<Rgba>>)};
            //let ba: Box<dyn std::any::Any> = unsafe {Box::from_raw(p[2].as_ptr())};
            //let render:Box<dyn Render<Rgba>>  = *ba.downcast::<Box<dyn Render<Rgba>>>().unwrap();
            let render = p[2].borrow_mut().downcast_mut::<Option<Rc<dyn Render<Rgba>>>>().unwrap().take().unwrap();
            sequence = sequence.append(time, restart, render);
        }
        r(Some(Rc::new(sequence) as Rc<dyn Render<Rgba>>))
    }) as MyFn));
    env
}