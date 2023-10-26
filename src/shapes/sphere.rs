use std::vec;

use crate::{

    ray::Ray,
    tuple::Tuple,
};

#[derive(PartialEq, Debug, Clone)]
pub struct Sphere {
    radius: f32,
    center: Tuple,
}
impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            radius: 1.0,
            center: Tuple::new_point(0.0, 0.0, 0.0),
        }
    }
}
impl Sphere {

    pub(crate) fn local_intersect(&self, other: &Ray) -> /*Intersections*/ Vec<f32> {
        let sphere_to_ray = other.origin - Tuple::new_point(0.0, 0.0, 0.0);
        let a = other.direction * other.direction;
        let b = 2.0 * (other.direction * sphere_to_ray);
        let c = sphere_to_ray * sphere_to_ray - 1.0;
        let discriminant: f32 = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            // return Intersections::new(Vec::new());
            return vec![]
        }
        // Intersections::new(vec![
        //     Intersection {
        //         object_ref: Box::new(),
        //         time: (-b - discriminant.sqrt()) / (2.0 * a),
        //     },
        //     Intersection {
        //         object_ref: Box::new(Object::Sphere(self.clone())),
        //         time: (-b + discriminant.sqrt()) / (2.0 * a),
        //     },
        // ])
        vec![(-b - discriminant.sqrt()) / (2.0 * a),(-b + discriminant.sqrt()) / (2.0 * a)]
    }

    pub(crate) fn normal_at(&self, point: Tuple) -> Tuple {
        let object_point = point;
        let object_normal = object_point - Tuple::new_point(0.0, 0.0, 0.0);
        object_normal.normalize()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}
