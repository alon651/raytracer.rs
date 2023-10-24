use crate::color::Color;
use crate::intersections::{Intersectable, Intersections};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::patterns::Pattern;
use crate::plane::Plane;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

impl Intersectable for Object {
    fn get_transform(&self) -> &Matrix {
        match self {
            Object::Sphere(ref s) => s.get_transform(),
            Object::Plane(ref p) => p.get_transform(),
        }
    }

    fn get_inverse(&self) -> &Matrix {
        match self {
            Object::Sphere(ref s) => s.get_inverse(),
            Object::Plane(ref p) => p.get_inverse(),
        }
    }

    fn local_intersect(&self, ray: &Ray) -> Intersections {
        match self {
            Object::Sphere(ref s) => s.local_intersect(ray),
            Object::Plane(ref p) => p.local_intersect(ray),
        }
    }

    fn get_material(&self) -> &Material {
        match self {
            Object::Sphere(ref s) => s.get_material(),
            Object::Plane(ref p) => p.get_material(),
        }
    }

    fn set_transform(&mut self, t: Matrix) {
        match self {
            Object::Sphere(ref mut s) => s.set_transform(t),
            Object::Plane(ref mut p) => p.set_transform(t),
        }
    }

    fn normal_at(&self, point: Tuple) -> Tuple {
        let w = match self {
            Object::Sphere(ref s) => s.normal_at(point),
            Object::Plane(ref p) => p.normal_at(point),
        };

        let mut w = &self.get_inverse().transpose() * w;
        w.w = 0.;
        w.normalize()
    }
}
impl Object {
    pub fn stripe_at_object(&self, pattern: &Pattern, world_point: Tuple) -> Color {
        let object_point = self.get_inverse() * world_point;
        let pattern_point = &pattern.inverse_transform * object_point;
        pattern.pattern_at(pattern_point)
    }
}
