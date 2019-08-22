#[derive(Debug, Copy, Clone)]
pub enum PointType {
    Constant,
    Linear,
    Bezier,
}

#[derive(Clone)]
pub struct Path {
    pub points: Vec<(f64, f64, PointType)>
}

impl Path {
    pub fn new(first_value: f64) -> Self {
        Path {
            points: vec![(0.0, first_value, PointType::Constant)]
        }
    }

    pub fn append(&mut self, d_time: f64, value: f64, point_type: PointType) -> &mut Self {
        assert!(0.0 <= d_time);
        self.points.push((self.points.last().unwrap().0 + d_time, value, point_type));
        self
    }

    pub fn get_value(&self, time: f64) -> f64 {
        if time < self.points[0].0 {
            return self.points[0].1;
        }
        for w in self.points.windows(2) {
            let (left, right) = (w[0], w[1]);
            if left.0 <= time && time < right.0 {
                return match right.2 {
                    PointType::Constant => left.1,
                    PointType::Linear => {
                        let v = (time - left.0) / (right.0 - left.0);
                        left.1 * (1.0 - v) + right.1 * v
                    },
                    PointType::Bezier => right.1 // TODO
                };
            }
        }
        println!("oops");
        self.points.last().unwrap().1
    }
}

#[test]
fn path_test () {
    let mut path = Path::new(0.0);
    path.append(1.0, 1.0, PointType::Constant)
        .append(1.0, 2.0, PointType::Linear);
    assert_eq!(path.get_value(-0.5), 0.0);
    assert_eq!(path.get_value(0.5), 0.0);
    assert_eq!(path.get_value(1.5), 1.5);
    assert_eq!(path.get_value(2.5), 2.0);
}
