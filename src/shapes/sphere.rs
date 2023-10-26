use std::vec;

use crate::{ray::Ray, tuple::Tuple};

#[derive(PartialEq, Debug, Clone)]
pub struct Sphere {
}
impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
        }
    }
}
impl Sphere {
    pub(crate) fn local_intersect(&self, other: &Ray) -> Vec<f32> {
        let sphere_to_ray = other.origin - Tuple::new_point(0.0, 0.0, 0.0);
        let a = other.direction * other.direction;
        let b = 2.0 * (other.direction * sphere_to_ray);
        let c = sphere_to_ray * sphere_to_ray - 1.0;
        let discriminant: f32 = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return vec![];
        }

        vec![
            (-b - discriminant.sqrt()) / (2.0 * a),
            (-b + discriminant.sqrt()) / (2.0 * a),
        ]
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
