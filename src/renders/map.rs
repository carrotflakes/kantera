use crate::render::{Res, Render, RenderOpt};

pub struct Map<T: Copy> {
    pub render: Box<Render<T>>,
    pub map: Box<Fn(usize, usize, &mut [T])>
}

impl <T: Copy> Render<T> for Map<T> {
    fn sample(&self, u: f64, v: f64, time: f64, res: Res) -> T {
        unimplemented!();
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [T]) {
        self.render.render(ro, buffer);

        let RenderOpt {u_res, v_res, frame_range, ..} = ro;
        let frame_size = *u_res * *v_res;
        for f in 0..(frame_range.end - frame_range.start) as usize {
            (self.map)(*u_res, *v_res, &mut buffer[f * frame_size..(f + 1) * frame_size]);
        }
    }

}
