use crate::traits::intersect::Intersect;
use crate::tuple::Tuple;
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn position(self, t: f32) -> Tuple {
        self.origin + self.direction * t
    }
    pub fn intersect<T: Intersect>(&self, other: T) -> Vec<f32> {
        other.calculate_intersection(&self)
    }
}
