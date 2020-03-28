use std::rc::Rc;
use std::ops::Range;
use serde::Serialize;
use kantera::{
    pixel::Rgba,
    render::Render,
    script::{Symbol, Val, ValInterface, SRange, Runtime}
};

pub fn mount(src: &str, position: i32, mut rt: Runtime) -> Option<Sequencer> {
    let vals = rt.r(src).ok()?;
    for i in 0..vals.len() {
        if let Some(mut s) = find_sequencer(&vals[i], position) {
            let mut vec = Vec::from(&vals[..i]);
            vec.push(s.val.ref_as::<Vec<Val>>()?[1].clone());
            let evaled = rt.e(vec).ok()?;
            let clips = evaled.ref_as::<Vec<Val>>()?;
            for i in 0..s.clips.len() {
                s.clips[i].duration = clips[i].ref_as::<Vec<Val>>()?[2].ref_as::<Rc<dyn Render<Rgba>>>()?.duration();
            }
            return Some(s);
        }
    }
    None
}

#[derive(Serialize)]
pub struct Clip {
    range: Range<i32>,
    start_range: Range<i32>,
    z_range: Range<i32>,
    render_range: Range<i32>,
    duration: f64
}

#[derive(Serialize)]
pub struct Sequencer {
    #[serde(skip_serializing)]
    val: Val,
    range: Range<i32>,
    clips: Vec<Clip>
}

fn find_sequencer(val: &Val, position: i32) -> Option<Sequencer> {
    fn srange_to_range(sr: &SRange) -> Range<i32> {
        sr.start..sr.end
    };
    let sr = val.get_meta::<SRange>()?;
    if !(sr.start <= position && position < sr.end) {
        return None;
    }
    let vec = val.ref_as::<Vec<Val>>()?;
    for v in vec.iter() {
        if let Some(x) = find_sequencer(v, position) {
            return Some(x);
        }
    }
    let s = vec[0].ref_as::<Symbol>()?;
    if s.0.as_str() != "sequencer" {
        return None;
    }
    let vec = vec[1].ref_as::<Vec<Val>>()?;
    if vec[0].ref_as::<Symbol>()?.0.as_str() != "vec" {
        return None;
    }
    let mut clips = Vec::new();
    for v in vec.iter().skip(1) {
        let vec = v.ref_as::<Vec<Val>>()?;
        if vec[0].ref_as::<Symbol>()?.0.as_str() != "vec" {
            return None;
        }
        vec[1].ref_as::<f64>()?;
        vec[2].ref_as::<i32>()?;
        clips.push(Clip {
            range: srange_to_range(v.get_meta::<SRange>()?),
            start_range: srange_to_range(vec[1].get_meta::<SRange>()?),
            z_range: srange_to_range(vec[2].get_meta::<SRange>()?),
            render_range: srange_to_range(vec[3].get_meta::<SRange>()?),
            duration: std::f64::INFINITY
        });
    }
    Some(Sequencer {
        val: val.clone(),
        range: srange_to_range(sr),
        clips
    })
}
