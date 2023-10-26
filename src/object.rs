use crate::color::Color;
use crate::intersections::{Intersectable, Intersections};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::patterns::Pattern;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::shapes::Shape;
#[derive(PartialEq, Debug, Clone)]
pub struct Object {
    shape:Shape,
    pub transform: Matrix,
    pub material: Material,
    pub inverse: Matrix,
}

impl Object {

    pub(crate) fn local_intersect(&self, ray: &Ray) -> Intersections {
        match self.shape {
            Shape::Sphere(ref s) => s.local_intersect(ray),
            Shape::Plane(ref p) => p.local_intersect(ray),
        }
    }

    fn set_transform(&mut self, t: Matrix) {
            self.transform = t.clone();
            self.inverse = t.inverse();

    }

    pub(crate) fn normal_at(&self, point: Tuple) -> Tuple {
        let point = &self.inverse * point;
        let w = match self.shape {
            Shape::Sphere(ref s) => s.normal_at(point),
            Shape::Plane(ref p) => p.normal_at(point),
        };

        let mut w = &self.inverse.transpose() * w;
        w.w = 0.;
        w.normalize()
    }
}
impl Object {
    pub fn stripe_at_object(&self, pattern: &Pattern, world_point: Tuple) -> Color {
        let object_point = &self.inverse * world_point;
        let pattern_point = &pattern.inverse_transform * object_point;
        pattern.pattern_at(pattern_point)
    }
}
