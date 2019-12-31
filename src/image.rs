#[derive(Debug)]
pub struct Image<T> {
    pub width: usize,
    pub height: usize,
    pub vec: Vec<T>
}

impl<T: Copy> Image<T> {
    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> Image<U> {
        Image {
            width: self.width,
            height: self.height,
            vec: self.vec.iter().map(f).collect()
        }
    }
}
