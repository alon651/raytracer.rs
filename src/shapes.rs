pub mod plane;
pub mod sphere;
pub mod cube;

use crate::ray::Ray;
use crate::tuple::Tuple;
use plane::Plane;
use sphere::Sphere;
use cube::Cube;

#[derive(PartialEq, Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
}

impl Shape {
    pub(crate) fn intersect(&self, ray: &Ray) -> Vec<f32> {
        match self {
            Shape::Sphere(ref shape) => {
                return shape.local_intersect(ray);
            }
            Shape::Plane(ref shape) => {
                return shape.local_intersect(ray);
            }
            Shape::Cube(ref shape) => {
                return shape.local_intersect(ray);
            }
        }
    }
}

impl Shape {
    pub fn normal_at(&self, point: Tuple) -> Tuple {
        let w = match self {
            Shape::Sphere(ref s) => s.normal_at(point),
            Shape::Plane(ref p) => p.normal_at(point),
            Shape::Cube(ref c) => c.normal_at(point)
        };
        w
    }
}
