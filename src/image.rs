#[derive(Debug)]
pub struct Image<T> {
    pub width: usize,
    pub height: usize,
    pub vec: Vec<T>
}
