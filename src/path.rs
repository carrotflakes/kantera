use crate::v::V;

#[derive(Debug, Clone, Copy)]
pub enum Point<T: Clone> {
    Constant,
    Linear,
    Bezier(T, T)
}

#[derive(Debug, Clone)]
pub struct Path<T: V> {
    pub points: Vec<(f64, T, Point<T>)>
}

impl<T: V> Path<T> {
    pub fn new(first_value: T) -> Self {
        Path {
            points: vec![(0.0, first_value, Point::Constant)]
        }
    }

    pub fn append(mut self, d_time: f64, value: T, point_type: Point<T>) -> Self {
        assert!(0.0 <= d_time);
        self.points.push((self.points.last().unwrap().0 + d_time, value, point_type));
        self
    }

    pub fn get_value(&self, time: f64) -> T {
        if time < self.points[0].0 {
            return self.points[0].1;
        }
        for w in self.points.windows(2) {
            let (left, right) = (w[0], w[1]);
            if left.0 <= time && time < right.0 {
                return match right.2 {
                    Point::Constant => left.1,
                    Point::Linear => {
                        let v = (time - left.0) / (right.0 - left.0);
                        left.1 * (1.0 - v).into() + right.1 * v.into()
                    },
                    Point::Bezier(right_handle, _) => {
                        let left_handle = match left.2 {
                            Point::Bezier(_, h) => h,
                            _ => left.1
                        };
                        let v = (time - left.0) / (right.0 - left.0);
                        let v1 = (1.0 - v).into();
                        let v2 = v.into();
                        (left.1 + left_handle * v2) * v1 +
                            (right_handle * v1 + right.1) * v2
                    }
                };
            }
        }
        self.points.last().unwrap().1
    }
}

// trait Timed

#[test]
fn path_test () {
    let path = Path::new(0.0)
        .append(1.0, 1.0, Point::Constant)
        .append(1.0, 2.0, Point::Linear);
    assert_eq!(path.get_value(-0.5), 0.0);
    assert_eq!(path.get_value(0.5), 0.0);
    assert_eq!(path.get_value(1.5), 1.5);
    assert_eq!(path.get_value(2.5), 2.0);

    use crate::v::Vec2;
    let path = Path::new(Vec2(0.0, 2.0))
        .append(1.0, Vec2(1.0, 0.0), Point::Constant)
        .append(1.0, Vec2(1.0, 2.0), Point::Linear);
    assert_eq!(path.get_value(1.5), Vec2(1.0, 1.0));
}
