pub struct Buffer<T> {
    pub width: usize,
    pub height: usize,
    pub frame_num: usize,
    pub framerate: usize,
    pub vec: Vec<T>
}
