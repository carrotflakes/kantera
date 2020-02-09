use crate::render::{Res, Render, RenderOpt};

pub struct Map<T: Copy, R: Render<T>> {
    pub render: R,
    pub map: Box<dyn Fn(usize, usize, &mut [T])>
}

impl <T: Copy, R: Render<T>> Render<T> for Map<T, R> {
    fn sample(&self, _u: f64, _v: f64, _time: f64, _res: Res) -> T {
        unimplemented!();
    }

    fn render(&self, ro: &RenderOpt, buffer: &mut [T]) {
        self.render.render(ro, buffer);

        let RenderOpt {res_x, res_y, frame_range, ..} = ro;
        let frame_size = *res_x * *res_y;
        for f in 0..(frame_range.end - frame_range.start) as usize {
            (self.map)(*res_x, *res_y, &mut buffer[f * frame_size..(f + 1) * frame_size]);
        }
    }

}
