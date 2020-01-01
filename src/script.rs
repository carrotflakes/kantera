use std::rc::Rc;
pub use gluten::{
    data::*,
    reader::Reader,
    core::{eval, Env}
};
use gluten::reader::make_default_atom_reader;
use crate::{
    image::Image,
    pixel::Rgba,
    render::Render,
    path::{Path, Point},
    v::{Vec2, Vec3}
};

pub fn make_reader() -> Reader {
    let mut default_atom_reader = make_default_atom_reader();
    Reader::new(Box::new(move |s: String| -> Result<Val, String> {
        if let Ok(v) = s.parse::<i32>() {
            return Ok(r(v));
        }
        if let Ok(v) = s.parse::<f64>() {
            return Ok(r(v));
        }
        default_atom_reader(s)
    }))
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
    env.insert("+".to_string(), r(Box::new(|vec: Vec<Val>| -> Val {
        fn f<T: num_traits::Num + Copy + 'static>(vec: &Vec<Val>) -> Option<Val> {
            let mut acc = T::zero();
            for rv in vec.iter() {
                acc = acc + *rv.borrow().downcast_ref::<T>()?;
            }
            Some(r(acc))
        }
        f::<f64>(&vec).or_else(|| f::<i32>(&vec)).or_else(|| f::<Vec2<f64>>(&vec)).or_else(|| f::<Vec3<f64>>(&vec)).unwrap()
    }) as MyFn));
    env.insert("-".to_string(), r(Box::new(|vec: Vec<Val>| -> Val {
        fn f<T: num_traits::Num + Copy + 'static>(vec: &Vec<Val>) -> Option<Val> {
            let mut acc = *vec[0].borrow().downcast_ref::<T>()?;
            for rv in vec.iter().skip(1) {
                acc = acc - *rv.borrow().downcast_ref::<T>()?;
            }
            Some(r(acc))
        }
        f::<f64>(&vec).or_else(|| f::<i32>(&vec)).or_else(|| f::<Vec2<f64>>(&vec)).or_else(|| f::<Vec3<f64>>(&vec)).unwrap()
    }) as MyFn));
    env.insert("*".to_string(), r(Box::new(|vec: Vec<Val>| -> Val {
        fn f<T: num_traits::Num + Copy + 'static>(vec: &Vec<Val>) -> Option<Val> {
            let mut acc = T::one();
            for rv in vec.iter() {
                acc = acc * *rv.borrow().downcast_ref::<T>()?;
            }
            Some(r(acc))
        }
        f::<f64>(&vec).or_else(|| f::<i32>(&vec)).or_else(|| f::<Vec2<f64>>(&vec)).or_else(|| f::<Vec3<f64>>(&vec)).unwrap()
    }) as MyFn));
    env.insert("/".to_string(), r(Box::new(|vec: Vec<Val>| -> Val {
        fn f<T: num_traits::Num + Copy + 'static>(vec: &Vec<Val>) -> Option<Val> {
            let mut acc = *vec[0].borrow().downcast_ref::<T>()?;
            for rv in vec.iter().skip(1) {
                acc = acc / *rv.borrow().downcast_ref::<T>()?;
            }
            Some(r(acc))
        }
        f::<f64>(&vec).or_else(|| f::<i32>(&vec)).or_else(|| f::<Vec2<f64>>(&vec)).or_else(|| f::<Vec3<f64>>(&vec)).unwrap()
    }) as MyFn));
    env.insert("stringify".to_string(), r(Box::new(|vec: Vec<Val>| -> Val {
        fn f<T: std::fmt::Debug + 'static>(vec: &Vec<Val>) -> Option<Val> {
            Some(r(format!("{:?}", vec[0].borrow().downcast_ref::<T>()?)))
        }
        f::<String>(&vec).or_else(|| f::<Symbol>(&vec))
        .or_else(|| f::<f64>(&vec)).or_else(|| f::<i32>(&vec)).or_else(|| f::<Vec2<f64>>(&vec)).or_else(|| f::<Vec3<f64>>(&vec))
        .or_else(|| f::<Rgba>(&vec))
        .unwrap()
    }) as MyFn));
    env.insert("rgb".to_string(), r(Box::new(|vec: Vec<Val>| {
        use regex::Regex;
        if let Some(string) = vec[0].borrow().downcast_ref::<String>() {
            let re = Regex::new(r"#([\da-fA-F]{2})([\da-fA-F]{2})([\da-fA-F]{2})").unwrap();
            if let Some(cap) = re.captures(string) {
                fn f(s: &str) -> f64 {
                    let mut cs = s.chars();
                    (cs.next().unwrap().to_digit(16).unwrap() * 16 + cs.next().unwrap().to_digit(16).unwrap()) as f64 / 255.0
                }
                r(Rgba(
                    f(&cap[1]),
                    f(&cap[2]),
                    f(&cap[3]),
                    1.0,
                ))
            } else {
                panic!("invalid RGB string");
            }
        } else {
            r(Rgba(
                *vec[0].borrow().downcast_ref::<f64>().unwrap(),
                *vec[1].borrow().downcast_ref::<f64>().unwrap(),
                *vec[2].borrow().downcast_ref::<f64>().unwrap(),
                1.0
            ))
        }
    }) as MyFn));
    env.insert("rgba".to_string(), r(Box::new(|vec: Vec<Val>| {
        use regex::Regex;
        if let Some(string) = vec[0].borrow().downcast_ref::<String>() {
            let re = Regex::new(r"#([\da-fA-F]{2})([\da-fA-F]{2})([\da-fA-F]{2})([\da-fA-F]{2})").unwrap();
            if let Some(cap) = re.captures(string) {
                fn f(s: &str) -> f64 {
                    let mut cs = s.chars();
                    (cs.next().unwrap().to_digit(16).unwrap() * 16 + cs.next().unwrap().to_digit(16).unwrap()) as f64 / 255.0
                }
                r(Rgba(
                    f(&cap[1]),
                    f(&cap[2]),
                    f(&cap[3]),
                    f(&cap[4]),
                ))
            } else {
                panic!("invalid RGBA string");
            }
        } else {
            r(Rgba(
                *vec[0].borrow().downcast_ref::<f64>().unwrap(),
                *vec[1].borrow().downcast_ref::<f64>().unwrap(),
                *vec[2].borrow().downcast_ref::<f64>().unwrap(),
                *vec[3].borrow().downcast_ref::<f64>().unwrap()
            ))
        }
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
        let default = *vec[1].borrow().downcast_ref::<Rgba>().unwrap();
        r(Some(Rc::new(crate::renders::image_render::ImageRender {
            image: image,
            sizing: crate::renders::image_render::Sizing::Contain,
            default: default
        }) as Rc<dyn Render<Rgba>>))
    }) as MyFn));
    env.insert("text_to_image".to_string(), r(Box::new(|vec: Vec<Val>| {
        let string = vec[0].borrow().downcast_ref::<String>().unwrap().clone();
        use crate::{text::{Font, render}};
        let font_path = "../IPAexfont00401/ipaexg.ttf"; // TODO
        let bytes = std::fs::read(font_path).unwrap();
        let font = Font::from_bytes(&bytes).unwrap();
        // TODO: font size
        r(Rc::new(render(&font, &string).map(|v| Rgba(0.0, 0.0, 0.0, *v))))
    }) as MyFn));
    env.insert("composite".to_string(), r(Box::new(|vec: Vec<Val>| {
        use crate::renders::composite::{Composite, CompositeMode};
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
    fn vec_to_vec2<T: 'static + num_traits::Num + Copy + From<f64>>(val: &Val) -> Vec2<T> {
        let val = val.borrow();
        let vec = val.downcast_ref::<Vec<Val>>().unwrap();
        let a = *vec[0].borrow().downcast_ref::<T>().unwrap();
        let b = *vec[1].borrow().downcast_ref::<T>().unwrap();
        Vec2(a, b)
    }
    fn vec_to_vec3<T: 'static + num_traits::Num + Copy + From<f64>>(val: &Val) -> Vec3<T> {
        let val = val.borrow();
        let vec = val.downcast_ref::<Vec<Val>>().unwrap();
        let a = *vec[0].borrow().downcast_ref::<T>().unwrap();
        let b = *vec[1].borrow().downcast_ref::<T>().unwrap();
        let c = *vec[2].borrow().downcast_ref::<T>().unwrap();
        Vec3(a, b, c)
    }
    env.insert("path".to_string(), r(Box::new(|vec: Vec<Val>| {
        let mut it = vec.into_iter();
        fn build_path<T: 'static + crate::lerp::Lerp>(first_value: T, it: impl Iterator<Item = Val>, vectorize: &impl Fn(&Val) -> T) -> Val {
            let mut path = Path::new(first_value);
            for rp in it {
                let rp = rp.borrow();
                let p = rp.downcast_ref::<Vec<Val>>().unwrap();
                let d_time = *p[0].borrow().downcast_ref::<f64>().unwrap();
                let vec = vectorize(&p[1]);
                let point = match p[2].borrow().downcast_ref::<String>().unwrap().as_str() {
                    "constant" => Point::Constant,
                    "linear" => Point::Linear,
                    "bezier" => Point::Bezier(vectorize(&p[3]), vectorize(&p[4])),
                    _ => panic!("invalid point type")
                };
                path = path.append(d_time, vec, point);
            }
            r(path)
        }
        if let Some(first_value) = it.next() {
            let v = first_value.borrow();
            if let Some(v) = v.downcast_ref::<f64>() {
                return build_path(*v, it, &|val| *val.borrow().downcast_ref::<f64>().unwrap());
            } else if let Some(vec) = v.downcast_ref::<Vec<Val>>() {
                match vec.len() {
                    2 => {
                        return build_path(vec_to_vec2::<f64>(&first_value), it, &vec_to_vec2);
                    },
                    3 => {
                        return build_path(vec_to_vec3::<f64>(&first_value), it, &vec_to_vec3);
                    },
                    _ => {}
                }
            }
            panic!("illegal path arguments")
        } else {
            panic!("path requires at least one argument")
        }
    }) as MyFn));
    env.insert("transform".to_string(), r(Box::new(|vec: Vec<Val>| {
        use crate::{renders::transform::{Transform, path_to_transformer}};
        let render = vec[0].borrow_mut().downcast_mut::<Option<Rc<dyn Render<Rgba>>>>().unwrap().take().unwrap();
        let translation_path = vec[1].borrow_mut().downcast_mut::<Path<Vec2<f64>>>().unwrap().clone();
        let scale_path = vec[2].borrow_mut().downcast_mut::<Path<Vec2<f64>>>().unwrap().clone();
        let rotation_path = vec[3].borrow_mut().downcast_mut::<Path<f64>>().unwrap().clone();
        r(Some(Rc::new(Transform::new(
            render,
            path_to_transformer(translation_path, scale_path, rotation_path)
        )) as Rc<dyn Render<Rgba>>))
    }) as MyFn));
    env
}
