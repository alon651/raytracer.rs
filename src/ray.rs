use crate::intersections::{Intersectable, Intersections};
use crate::matrix::Matrix;
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
    pub fn intersect(&self, other: &dyn Intersectable) -> Intersections {
        let ray2 = self.transform(other.getTransform().inverse());
        other.intersect(&ray2)
    }
    pub fn transform(&self, transformation: Matrix) -> Ray {
        Ray {
            origin: &transformation * self.origin,
            direction: &transformation * self.direction,
        }
    }
}
