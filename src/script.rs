use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Write;
use gluten::{
    reader::{Reader, default_atom_reader},
    core::{eval, Env, macro_expand, Macro},
    StringPool,
    val_helper::Get
};
pub use gluten::{
    data::*,
    error::GlutenError
};
use crate::{
    image::Image,
    pixel::Rgba,
    render::Render,
    audio_buffer::AudioBuffer,
    audio_render::AudioRender,
    audio_renders,
    path::{Path, Point},
    timed::Timed,
    lerp::Lerp,
    v::{Vec2, Vec3},
    interpolation
};

pub struct Runtime(Env);

impl Runtime {
    pub fn new() -> Runtime {
        let reader = Reader::new(Box::new(atom_reader));
        let mut env = Env::new(Rc::new(RefCell::new(reader)));
        gluten::special_operators::insert_all(&mut env);
        let mut rt = Runtime(env);
        init_runtime(&mut rt);
        rt
    }

    pub fn insert(&mut self, str: &str, val: Val) {
        let sym = self.0.reader().borrow_mut().intern(str);
        self.0.insert(sym, val);
    }

    pub fn get(&self, str: &str) -> Option<Val> {
        let sym = self.0.reader().borrow().try_intern(str)?;
        self.0.get(&sym)
    }

    pub fn re(&mut self, str: &str) -> Result<Val, GlutenError>{
        let forms = self.0.reader().borrow_mut().parse_top_level(str)?;
        let mut last_val = None;
        for form in forms {
            let form = macro_expand(&mut self.0, form)?;
            last_val = Some(eval(self.0.clone(), form)?);
        }
        last_val.ok_or(GlutenError::Str("no form".to_string()))
    }
}

fn atom_reader(sp: &mut StringPool, s: &str) -> Result<Val, GlutenError> {
    if let Ok(v) = s.parse::<i32>() {
        return Ok(r(v));
    }
    if let Ok(v) = s.parse::<f64>() {
        return Ok(r(v));
    }
    default_atom_reader(sp, s)
}

fn write_val<T: Write>(write: &mut T, val: &Val) {
    if let Some(s) = val.downcast_ref::<Symbol>() {
        write!(write, "{}", s.0.as_ref()).unwrap();
    } else if let Some(s) = val.downcast_ref::<String>() {
        write!(write, "{:?}", s).unwrap();
    } else if let Some(s) = val.downcast_ref::<i32>() {
        write!(write, "{:?}", s).unwrap();
    } else if let Some(s) = val.downcast_ref::<f64>() {
        write!(write, "{:?}", s).unwrap();
    } else if let Some(vec) = val.downcast_ref::<Vec<Val>>() {
        write!(write, "(").unwrap();
        let mut first = true;
        for val in vec {
            if first {
                first = false;
            } else {
                write!(write, " ").unwrap();
            }
            write_val(write, val);
        }
        write!(write, ")").unwrap();
    } else {
        write!(write, "#?#").unwrap();
    }
}

fn init_runtime(rt: &mut Runtime) {
    rt.insert("true", r(true));
    rt.insert("false", r(false));
    rt.insert("first", r(Box::new(|vec: Vec<Val>| {
        vec.get(0).cloned().ok_or_else(|| GlutenError::Str("no argument given".to_owned()))
    }) as NativeFn));
    rt.insert("vec", r(Box::new(|vec: Vec<Val>| {
        Ok(r(vec))
    }) as NativeFn));
    rt.insert("+", r(Box::new(|vec: Vec<Val>| {
        fn f<T: num_traits::Num + Copy + 'static>(vec: &Vec<Val>) -> Option<Val> {
            let mut acc = T::zero();
            for rv in vec.iter() {
                acc = acc + *rv.downcast_ref::<T>()?;
            }
            Some(r(acc))
        }
        f::<f64>(&vec).or_else(|| f::<i32>(&vec)).or_else(|| f::<Vec2<f64>>(&vec)).or_else(|| f::<Vec3<f64>>(&vec)).ok_or_else(|| GlutenError::Str("arguments mismatch".to_owned()))
    }) as NativeFn));
    rt.insert("-", r(Box::new(|vec: Vec<Val>| {
        fn f<T: num_traits::Num + Copy + 'static>(vec: &Vec<Val>) -> Option<Val> {
            let mut acc = *vec.get(0)?.downcast_ref::<T>()?;
            for rv in vec.iter().skip(1) {
                acc = acc - *rv.downcast_ref::<T>()?;
            }
            Some(r(acc))
        }
        f::<f64>(&vec).or_else(|| f::<i32>(&vec)).or_else(|| f::<Vec2<f64>>(&vec)).or_else(|| f::<Vec3<f64>>(&vec)).ok_or_else(|| GlutenError::Str("arguments mismatch".to_owned()))
    }) as NativeFn));
    rt.insert("*", r(Box::new(|vec: Vec<Val>| {
        fn f<T: num_traits::Num + Copy + 'static>(vec: &Vec<Val>) -> Option<Val> {
            let mut acc = T::one();
            for rv in vec.iter() {
                acc = acc * *rv.downcast_ref::<T>()?;
            }
            Some(r(acc))
        }
        f::<f64>(&vec).or_else(|| f::<i32>(&vec)).or_else(|| f::<Vec2<f64>>(&vec)).or_else(|| f::<Vec3<f64>>(&vec)).ok_or_else(|| GlutenError::Str("arguments mismatch".to_owned()))
    }) as NativeFn));
    rt.insert("/", r(Box::new(|vec: Vec<Val>| {
        fn f<T: num_traits::Num + Copy + 'static>(vec: &Vec<Val>) -> Option<Val> {
            let mut acc = *vec.get(0)?.downcast_ref::<T>()?;
            for rv in vec.iter().skip(1) {
                acc = acc / *rv.downcast_ref::<T>()?;
            }
            Some(r(acc))
        }
        f::<f64>(&vec).or_else(|| f::<i32>(&vec)).or_else(|| f::<Vec2<f64>>(&vec)).or_else(|| f::<Vec3<f64>>(&vec)).ok_or_else(|| GlutenError::Str("arguments mismatch".to_owned()))
    }) as NativeFn));
    rt.insert("PI", r(std::f64::consts::PI));
    rt.insert("sin", r(Box::new(|vec: Vec<Val>| {
        let v = vec.get_(0)?.copy_as::<f64>().ok_or_else(|| GlutenError::Str("arguments mismatch".to_owned()))?;
        Ok(r(v.sin()))
    }) as NativeFn));
    rt.insert("cos", r(Box::new(|vec: Vec<Val>| {
        let v = vec.get_(0)?.copy_as::<f64>().ok_or_else(|| GlutenError::Str("arguments mismatch".to_owned()))?;
        Ok(r(v.cos()))
    }) as NativeFn));
    rt.insert("stringify", r(Box::new(|vec: Vec<Val>| {
        fn f<T: std::fmt::Debug + 'static>(vec: &Vec<Val>) -> Option<Val> {
            Some(r(format!("{:?}", vec.get(0)?.downcast_ref::<T>()?)))
        }
        f::<String>(&vec).or_else(|| f::<Symbol>(&vec))
        .or_else(|| f::<f64>(&vec)).or_else(|| f::<i32>(&vec)).or_else(|| f::<Vec2<f64>>(&vec)).or_else(|| f::<Vec3<f64>>(&vec))
        .or_else(|| f::<Rgba>(&vec))
        .ok_or_else(|| GlutenError::Str("arguments mismatch".to_owned()))
    }) as NativeFn));
    rt.insert("rgb", r(Box::new(|vec: Vec<Val>| {
        use regex::Regex;
        if let Some(string) = vec[0].downcast_ref::<String>() {
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
                *vec[0].downcast_ref::<f64>().unwrap(),
                *vec[1].downcast_ref::<f64>().unwrap(),
                *vec[2].downcast_ref::<f64>().unwrap(),
                1.0
            ))
        }
    }) as MyFn));
    rt.insert("rgba", r(Box::new(|vec: Vec<Val>| {
        use regex::Regex;
        if let Some(string) = vec[0].downcast_ref::<String>() {
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
                *vec[0].downcast_ref::<f64>().unwrap(),
                *vec[1].downcast_ref::<f64>().unwrap(),
                *vec[2].downcast_ref::<f64>().unwrap(),
                *vec[3].downcast_ref::<f64>().unwrap()
            ))
        }
    }) as MyFn));
    rt.insert("plain", r(Box::new(|vec: Vec<Val>| {
        let first = vec.get_(0)?;
        if let Some(p) = first.copy_as::<Rgba>() {
            Ok(r(Rc::new(crate::renders::plain::Plain::new(p)) as Rc<dyn Render<Rgba>>))
        } else if let Some(p) = clone_timed(first) {
            Ok(r(Rc::new(crate::renders::plain::Plain::new(p.clone())) as Rc<dyn Render<Rgba>>))
        } else {
            Err(GlutenError::Str("arguments mismatch".to_owned()))
        }
    }) as NativeFn));
    rt.insert("clip", r(Box::new(|vec: Vec<Val>| {
        let render = vec.get_(0)?.clone_as::<Rc<dyn Render<Rgba>>>().ok_or_else(|| GlutenError::Str("type mismatch".to_owned()))?;
        let start = vec.get_(1)?.copy_as::<f64>().ok_or_else(|| GlutenError::Str("arguments mismatch".to_owned()))?;
        let end = vec.get_(2)?.copy_as::<f64>().ok_or_else(|| GlutenError::Str("arguments mismatch".to_owned()))?;
        Ok(r(Rc::new(crate::renders::clip::Clip::new(render, start, end)) as Rc<dyn Render<Rgba>>))
    }) as NativeFn));
    rt.insert("frame", r(Box::new(|vec: Vec<Val>| {
        use crate::renders::frame::{Frame, FrameType};
        let render = vec.get_(0)?.clone_as::<Rc<dyn Render<Rgba>>>().ok_or_else(|| GlutenError::Str("type mismatch".to_owned()))?;
        let frame_type = match vec.get_(1)?.downcast_ref::<Symbol>().unwrap().0.as_str() {
            "constant" => FrameType::Constant(vec.get_(2)?.copy_as::<Rgba>().ok_or_else(|| GlutenError::Str("type mismatch".to_owned()))?),
            "extend" => FrameType::Extend,
            "repeat" => FrameType::Repeat,
            "reflect" => FrameType::Reflect,
            _ => { return Err(GlutenError::Str(format!("invalid frame_type"))) }
        };
        Ok(r(Rc::new(Frame {render, frame_type}) as Rc<dyn Render<Rgba>>))
    }) as NativeFn));
    rt.insert("sequence", r(Box::new(|vec: Vec<Val>| {
        let mut sequence = crate::renders::sequence::Sequence::new();
        for p in vec.into_iter() {
            let p = p.downcast_ref::<Vec<Val>>().unwrap().clone();
            let time = *p[0].downcast_ref::<f64>().unwrap();
            let restart = *p[1].downcast_ref::<bool>().unwrap();
            let render = p[2].downcast_ref::<Rc<dyn Render<Rgba>>>().unwrap().clone();
            sequence = sequence.append(time, restart, render);
        }
        r(Rc::new(sequence) as Rc<dyn Render<Rgba>>)
    }) as MyFn));
    rt.insert("sequencer", r(Box::new(|vec: Vec<Val>| {
        let mut sequencer = crate::renders::sequencer::Sequencer::new(Rgba(0.0, 0.0, 0.0, 0.0)); // TODO
        for p in vec.get_(0)?.clone_as::<Vec<Val>>().ok_or_else(|| GlutenError::Str("type mismatch".to_owned()))? {
            let p = p.clone_as::<Vec<Val>>().ok_or_else(|| GlutenError::Str("type mismatch".to_owned()))?;
            let time = p.get_(0)?.copy_as::<f64>().ok_or_else(|| GlutenError::Str("type mismatch".to_owned()))?;
            let z = p.get_(1)?.copy_as::<i32>().ok_or_else(|| GlutenError::Str("type mismatch".to_owned()))?;
            let render = p.get_(2)?.clone_as::<Rc<dyn Render<Rgba>>>().ok_or_else(|| GlutenError::Str("type mismatch".to_owned()))?;
            sequencer = sequencer.append(time, z as usize, render);
        }
        Ok(r(Rc::new(sequencer) as Rc<dyn Render<Rgba>>))
    }) as NativeFn));
    rt.insert("image_render", r(Box::new(|vec: Vec<Val>| {
        let image = vec[0].downcast_ref::<Rc<Image<Rgba>>>().unwrap().clone();
        let default = *vec[1].downcast_ref::<Rgba>().unwrap();
        r(Rc::new(crate::renders::image_render::ImageRender {
            image: image,
            sizing: crate::renders::image_render::Sizing::Contain,
            default: default,
            interpolation: interpolation::Bilinear // TODO
        }) as Rc<dyn Render<Rgba>>)
    }) as MyFn));
    rt.insert("text_to_image", r(Box::new(|vec: Vec<Val>| {
        let string = vec[0].downcast_ref::<String>().unwrap().clone();
        let scale = *vec[1].downcast_ref::<f64>().unwrap();
        let font = vec.get(2).and_then(|v| v.downcast_ref::<Rc<crate::text::Font>>().cloned()).unwrap_or_else(|| {
            let font_path = "./tmp/IPAexfont00401/ipaexg.ttf";
            let bytes = std::fs::read(font_path).unwrap();
            Rc::new(crate::text::Font::from_bytes(bytes).unwrap())
        });
        r(Rc::new(crate::text::render(&font, scale as f32, &string).map(|v| Rgba(0.0, 0.0, 0.0, *v))))
    }) as MyFn));
    rt.insert("composite", r(Box::new(|vec: Vec<Val>| {
        use crate::renders::composite::{Composite, CompositeMode};
        let layers = vec.into_iter().map(|p| {
            let p = p.downcast_ref::<Vec<Val>>().unwrap().clone();
            let render = p[0].downcast_ref::<Rc<dyn Render<Rgba>>>().unwrap().clone();
            let mode = p[1].downcast_ref::<Symbol>().unwrap().0.to_owned();
            let mode = match mode.as_str() {
                "none" => CompositeMode::None,
                "normal" => CompositeMode::Normal(
                    p.get(2).and_then(clone_timed::<f64>).unwrap_or_else(|| Rc::new(1.0))
                ),
                _ => panic!("illegal CompositeMode")
            };
            (render, mode)
        }).collect();
        r(Rc::new(Composite {
            layers: layers
        }) as Rc<dyn Render<Rgba>>)
    }) as MyFn));
    fn vec_to_vec2<T: 'static + num_traits::Num + Lerp>(val: &Val) -> Vec2<T> {
        let val = val;
        let vec = val.downcast_ref::<Vec<Val>>().unwrap();
        let a = *vec[0].downcast_ref::<T>().unwrap();
        let b = *vec[1].downcast_ref::<T>().unwrap();
        Vec2(a, b)
    }
    fn vec_to_vec3<T: 'static + num_traits::Num + Lerp>(val: &Val) -> Vec3<T> {
        let val = val;
        let vec = val.downcast_ref::<Vec<Val>>().unwrap();
        let a = *vec[0].downcast_ref::<T>().unwrap();
        let b = *vec[1].downcast_ref::<T>().unwrap();
        let c = *vec[2].downcast_ref::<T>().unwrap();
        Vec3(a, b, c)
    }
    rt.insert("path", r(Box::new(|vec: Vec<Val>| {
        let mut it = vec.into_iter();
        fn build_path<T: 'static + Clone + Lerp>(first_value: T, it: impl Iterator<Item = Val>, vectorize: &impl Fn(&Val) -> T) -> Val {
            let mut path = Path::new(first_value);
            for rp in it {
                let rp = rp;
                let p = rp.downcast_ref::<Vec<Val>>().unwrap();
                let d_time = *p[0].downcast_ref::<f64>().unwrap();
                let vec = vectorize(&p[1]);
                let point = match p[2].downcast_ref::<Symbol>().unwrap().0.as_str() {
                    "constant" => Point::Constant,
                    "linear" => Point::Linear,
                    "bezier2" => Point::Bezier2(vectorize(&p[3])),
                    "bezier3" => Point::Bezier3(vectorize(&p[3]), vectorize(&p[4])),
                    _ => panic!("invalid point type")
                };
                path = path.append(d_time, vec, point);
            }
            r(Rc::new(path))
        }
        if let Some(v) = it.next() {
            if let Some(v) = v.downcast_ref::<f64>() {
                return build_path(*v, it, &|val| *val.downcast_ref::<f64>().unwrap());
            } else if let Some(v) = v.downcast_ref::<Rgba>() {
                return build_path(*v, it, &|val| *val.downcast_ref::<Rgba>().unwrap());
            } else if let Some(vec) = v.downcast_ref::<Vec<Val>>() {
                match vec.len() {
                    2 => {
                        return build_path(vec_to_vec2::<f64>(&v), it, &vec_to_vec2);
                    },
                    3 => {
                        return build_path(vec_to_vec3::<f64>(&v), it, &vec_to_vec3);
                    },
                    _ => {}
                }
            }
            panic!("illegal path arguments")
        } else {
            panic!("path requires at least one argument")
        }
    }) as MyFn));
    rt.insert("timed/cycle", r(Box::new(|vec: Vec<Val>| {
        use crate::timed::Cycle;
        fn f<T: 'static + Lerp>(vec: &Vec<Val>) -> Option<Val> {
            let timed = clone_timed(&vec[0])?;
            let duration = *vec[1].downcast_ref::<f64>().unwrap();
            Some(r(Rc::new(Cycle::new(timed, duration)) as Rc<dyn Timed<T>>))
        }
        f::<f64>(&vec).or_else(|| f::<Vec2<f64>>(&vec)).or_else(|| f::<Vec3<f64>>(&vec)).or_else(|| f::<Rgba>(&vec)).unwrap()
    }) as MyFn));
    rt.insert("timed/sin", r(Box::new(|vec: Vec<Val>| {
        use crate::timed::Sine;
        fn f<T: 'static + Clone + Timed<f64>>(vec: &Vec<Val>) -> Option<Val> {
            let initial_phase = *vec[0].downcast_ref::<f64>().unwrap();
            let frequency = vec[1].downcast_ref::<f64>().unwrap().clone();
            let amplitude = vec[2].downcast_ref::<T>().unwrap().clone();
            Some(r(Rc::new(Sine::new(initial_phase, frequency, amplitude)) as Rc<dyn Timed<f64>>))
        }
        f::<f64>(&vec).or_else(|| f::<Rc<dyn Timed<f64>>>(&vec)).unwrap()
    }) as MyFn));
    rt.insert("timed/add", r(Box::new(|vec: Vec<Val>| {
        fn f<T: 'static + Lerp>(vec: &Vec<Val>) -> Option<Val> {
            let a = clone_timed(&vec[0])?;
            let b = clone_timed(&vec[1])?;
            Some(r(Rc::new(crate::timed::Add::new(a, b)) as Rc<dyn Timed<T>>))
        }
        f::<f64>(&vec).or_else(|| f::<Vec2<f64>>(&vec)).or_else(|| f::<Vec3<f64>>(&vec)).unwrap()
    }) as MyFn));
    rt.insert("timed/mul", r(Box::new(|vec: Vec<Val>| {
        fn f<T: 'static + Lerp + std::ops::Mul<Output = T>>(vec: &Vec<Val>) -> Option<Val> {
            let a = clone_timed(&vec[0])?;
            let b = clone_timed(&vec[1])?;
            Some(r(Rc::new(crate::timed::Mul::new(a, b)) as Rc<dyn Timed<T>>))
        }
        f::<f64>(&vec).or_else(|| f::<Vec2<f64>>(&vec)).or_else(|| f::<Vec3<f64>>(&vec)).unwrap()
    }) as MyFn));
    rt.insert("timed/map_sin", r(Box::new(|vec: Vec<Val>| {
        use crate::timed::Map;
        fn f<T: 'static + Clone + Timed<f64>>(vec: &Vec<Val>) -> Option<Val> {
            let timed = vec[0].downcast_ref::<T>()?.clone();
            Some(r(Rc::new(Map::new(timed, |x| x.sin())) as Rc<dyn Timed<f64>>))
        }
        f::<f64>(&vec).or_else(|| f::<Rc<dyn Timed<f64>>>(&vec)).or_else(
            || Some(r(Rc::new(Map::new(clone_timed(&vec[0])?, |x| x.sin())) as Rc<dyn Timed<f64>>))
        ).unwrap()
    }) as MyFn));
    rt.insert("transform", r(Box::new(|vec: Vec<Val>| {
        use crate::{renders::transform::{Transform, timed_to_transformer}};
        let render = vec[0].downcast_ref::<Rc<dyn Render<Rgba>>>().unwrap().clone();
        fn get_timed_vec2(val: &Val) -> Rc<dyn Timed<Vec2<f64>>> {
            if let Some(timed) = clone_timed::<Vec2<f64>>(val) {
                timed
            } else {
                let val = val;
                let v = val.downcast_ref::<Vec<Val>>().unwrap();
                let a = *v[0].downcast_ref::<f64>().unwrap();
                let b = *v[1].downcast_ref::<f64>().unwrap();
                Rc::new(Vec2(a, b))
            }
        }
        fn get_timed_f64(val: &Val) -> Rc<dyn Timed<f64>> {
            if let Some(timed) = clone_timed::<f64>(val) {
                timed
            } else {
                Rc::new(val.downcast_ref::<f64>().unwrap().clone())
            }
        }
        let translation_timed = get_timed_vec2(&vec[1]);
        let scale_timed = get_timed_vec2(&vec[2]);
        let rotation_timed = get_timed_f64(&vec[3]);
        r(Rc::new(Transform::new(
            render,
            timed_to_transformer(translation_timed, scale_timed, rotation_timed)
        )) as Rc<dyn Render<Rgba>>)
    }) as MyFn));
    rt.insert("audio_buffer_render", r(Box::new(|vec: Vec<Val>| {
        let audio_buffer = vec[0].downcast_ref::<Rc<AudioBuffer<u16>>>().unwrap().clone();
        r(Rc::new(audio_renders::audio_buffer::AudioBufferRender {
            audio_buffer: audio_buffer,
            interpolation: interpolation::NearestNeighbor
        }) as Rc<dyn AudioRender>)
    }) as MyFn));
    rt.insert("audio_clip", r(Box::new(|vec: Vec<Val>| {
        let audio_render = vec[0].downcast_ref::<Rc<dyn AudioRender>>().unwrap().clone();
        r(Rc::new(audio_renders::audio_clip::AudioClip {
            audio_render: audio_render,
            gain: *vec[1].downcast_ref::<f64>().unwrap(),
            pan: *vec[2].downcast_ref::<f64>().unwrap(),
            start: *vec[3].downcast_ref::<f64>().unwrap(),
            duration: *vec[4].downcast_ref::<f64>().unwrap(),
            pitch: *vec[5].downcast_ref::<f64>().unwrap(),
            fadein: *vec[6].downcast_ref::<f64>().unwrap(),
            fadeout: *vec[7].downcast_ref::<f64>().unwrap()
        }) as Rc<dyn AudioRender>)
    }) as MyFn));
    rt.insert("audio_sequencer", r(Box::new(|vec: Vec<Val>| {
        let renders = vec.into_iter().map(|p| {
            let p = p.downcast_ref::<Vec<Val>>().unwrap().clone();
            let time = p[0].downcast_ref::<f64>().unwrap().clone();
            let render = p[1].downcast_ref::<Rc<dyn AudioRender>>().unwrap().clone();
            (time, render)
        }).collect();
        r(Rc::new(audio_renders::sequencer::Sequencer {renders}) as Rc<dyn AudioRender>)
    }) as MyFn));
    rt.insert("audio/timed", r(Box::new(|vec: Vec<Val>| {
        let timed = vec.get_(0)?.clone_as::<Rc<dyn Timed<f64>>>().ok_or_else(|| GlutenError::Str("arguments mismatch".to_owned()))?;
        Ok(r(Rc::new(timed) as Rc<dyn AudioRender>))
    }) as NativeFn));
    rt.insert("test_audio", r(Box::new(|_vec: Vec<Val>| {
        use crate::audio_renders::{note::Note, sequencer::Sequencer};
        fn note(dur: f64, nn: i32, vel: f64, pan: f64) -> Box<dyn AudioRender> {
            Box::new(Note {
                frequency: 440.0 * 2.0f64.powf((nn - 69) as f64 / 12.0),
                duration: dur,
                gain: vel,
                pan: pan
            })
        }
        //r(Rc::new(note(1.0, 60, 0.3, 1.0)) as Rc<dyn AudioRender>)
        r(Rc::new(Sequencer::new()
            .append(0.00, note(1.0, 60, 0.2, -1.0))
            .append(1.00, note(1.0, 64, 0.2, -1.0))
            .append(2.00, note(1.0, 62, 0.2, -1.0))
            .append(3.00, note(1.0, 67, 0.2, -1.0))
            .append(4.00, note(1.0, 60, 0.2, 1.0))
            .append(5.00, note(1.0, 64, 0.2, 1.0))
            .append(6.00, note(1.0, 62, 0.2, 1.0))
            .append(7.00, note(1.0, 67, 0.2, 1.0))
            .append(0.00, note(0.25, 72, 0.1, 0.0))
            .append(0.50, note(0.25, 72, 0.1, 0.0))
            .append(1.00, note(0.25, 72, 0.1, 0.0))
            .append(1.50, note(0.25, 72, 0.1, 0.0))
            .append(2.00, note(0.25, 72, 0.1, 0.0))
            .append(2.50, note(0.25, 72, 0.1, 0.0))
            .append(2.00, note(0.25, 72, 0.1, 0.0))
            .append(2.50, note(0.25, 72, 0.1, 0.0))
            .append(3.00, note(0.25, 72, 0.1, 0.0))
            .append(3.50, note(0.50, 74, 0.1, 0.0))
            .append(4.00, note(0.25, 72, 0.1, 0.0))
            .append(4.50, note(0.25, 72, 0.1, 0.0))
            .append(5.00, note(0.25, 72, 0.1, 0.0))
            .append(5.50, note(0.25, 72, 0.1, 0.0))
            .append(6.00, note(0.25, 72, 0.1, 0.0))
            .append(6.50, note(0.25, 72, 0.1, 0.0))
            .append(7.00, note(0.25, 72, 0.1, 0.0))
            .append(7.50, note(0.50, 74, 0.1, 0.0))) as Rc<dyn AudioRender>)
    }) as MyFn));
    rt.insert("path_to_image", r(Box::new(|vec: Vec<Val>| {
        use crate::path_to_image::{closed_path_rect, closed_path_to_image, expand_rect};
        let path = vec[0].downcast_ref::<Rc<Path<Vec2<f64>>>>().unwrap().clone();
        let line_width = 3.0f64;
        let rect = expand_rect(closed_path_rect(&path), line_width.ceil() as i32);
        r(Rc::new(closed_path_to_image(rect, Rgba(1.0, 0.0, 0.0, 1.0), Rgba(1.0, 1.0, 1.0, 1.0), line_width, &path)))
    }) as MyFn));
    rt.insert("import_image", r(Box::new(|vec: Vec<Val>| {
        let filepath = vec[0].downcast_ref::<String>().unwrap().clone();
        r(Rc::new(crate::image_import::load_image(&filepath)))
    }) as MyFn));
    #[cfg(feature = "ffmpeg")]
    rt.insert("import_audio", r(Box::new(|vec: Vec<Val>| {
        let filepath = vec[0].downcast_ref::<String>().unwrap().clone();
        r(Rc::new(crate::ffmpeg::import_audio(&filepath)))
    }) as MyFn));
    rt.insert("import_ttf", r(Box::new(|vec: Vec<Val>| {
        let filepath = vec[0].downcast_ref::<String>().unwrap().clone();
        let bytes = std::fs::read(filepath).unwrap();
        let font = crate::text::Font::from_bytes(bytes).unwrap();
        r(Rc::new(font))
    }) as MyFn));
    rt.insert("hash_map_get", r(Box::new(|vec: Vec<Val>| {
        let hash_map = vec[0].downcast_ref::<RefCell<std::collections::HashMap<String, Val>>>().unwrap().borrow_mut();
        let key = vec[1].downcast_ref::<String>().unwrap().clone();
        hash_map.get(&key).map(|x| x.clone()).unwrap_or_else(|| r(false))
    }) as MyFn));
    rt.insert("hash_map_set", r(Box::new(|vec: Vec<Val>| {
        let mut hash_map = vec[0].downcast_ref::<RefCell<std::collections::HashMap<String, Val>>>().unwrap().borrow_mut();
        let key = vec[1].downcast_ref::<String>().unwrap().clone();
        let val = vec[2].clone();
        hash_map.insert(key, val.clone());
        val
    }) as MyFn));
    rt.insert("or", r(Macro(Box::new(|env: &mut Env, vec: Vec<Val>| {
        let let_sym = r(env.reader().borrow_mut().intern("let"));
        let if_sym = r(env.reader().borrow_mut().intern("if"));
        let mut ret = vec.last().unwrap().clone();
        let mut i = 0;
        for val in vec.iter().rev().skip(1) {
            i += 1;
            let sym = r(env.reader().borrow_mut().intern(&format!("#gensym{}#", i)));
            ret = r(vec![
                let_sym.clone(),
                r(vec![r(vec![sym.clone(), val.clone()])]),
                r(vec![if_sym.clone(), sym.clone(), sym.clone(), ret])
            ]);
        }
        Ok(ret)
    }))));
    rt.insert("defmacro", r(Macro(Box::new(gluten::core::defmacro))));
    rt.insert("quasiquote", r(Macro(Box::new(gluten::quasiquote::quasiquote))));
    rt.insert("with_cache", r(Macro(Box::new(|env: &mut Env, vec: Vec<Val>| {
        let reader = env.reader();
        let mut reader = reader.borrow_mut();
        let rt_cache = env.get(&reader.intern("__rt_cache")).unwrap().clone();
        let mut key = String::new();
        write_val(&mut key, &vec[0]);
        Ok(r(vec![
            r(reader.intern("or")),
            r(vec![r(reader.intern("hash_map_get")), rt_cache.clone(), r(key.clone())]),
            r(vec![r(reader.intern("hash_map_set")), rt_cache, r(key), vec[0].clone()])
        ]))
    }))));
}

fn clone_timed<T: 'static + Lerp>(val: &Val) -> Option<Rc<dyn Timed<T>>> {
    val.downcast_ref::<Rc<dyn Timed<T>>>().cloned()
        .or_else(|| val.downcast_ref::<Rc<Path<T>>>().map(|x| x.clone() as Rc<dyn Timed<T>>))
        .or_else(|| val.downcast_ref::<T>().map(|x| Rc::new(*x) as Rc<dyn Timed<T>>))
}

trait FnArgs {
    fn get_(&self, i: usize) -> Result<&Val, GlutenError>;
}
impl FnArgs for Vec<Val> {
    fn get_(&self, i: usize) -> Result<&Val, GlutenError> {
        self.get(i).ok_or_else(|| GlutenError::Str("argument missing".to_owned()))
    }
}
