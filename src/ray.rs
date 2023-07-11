use crate::intersections::{Intersectable, Intersection, Intersections};
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
    pub fn intersect<'a>(&self, other: &'a dyn Intersectable) -> Intersections {
        other.intersect(self)
    }
}
