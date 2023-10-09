use crate::intersections::{Intersectable, Intersections};
use crate::matrix::Matrix;
use crate::object::Object;
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
    pub fn intersect<'a>(&'a self, other: &'a Object) -> Intersections {
        let ray2 = self.transform(other.get_transform().inverse());
        other.intersect(&ray2)
    }
    pub fn transform(&self, transformation: Matrix) -> Ray {
        Ray {
            origin: &transformation * self.origin,
            direction: &transformation * self.direction,
        }
    }
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }
}
