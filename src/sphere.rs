use std::vec;

use crate::{
    intersections::{Intersectable, Intersection, Intersections},
    ray::Ray,
    tuple::Tuple,
    utils::generateId,
};

#[derive(PartialEq, Debug)]
pub struct Sphere {
    radius: f32,
    center: Tuple,
    pub id: usize,
}
impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            radius: 1.0,
            center: Tuple::new_point(0.0, 0.0, 0.0),
            id: generateId(),
        }
    }
}
impl Intersectable for Sphere {
    fn intersect(&self, other: &Ray) -> Intersections {
        let sphere_to_ray = other.origin - Tuple::new_point(0.0, 0.0, 0.0);
        let a = other.direction * other.direction;
        let b = 2.0 * (other.direction * sphere_to_ray);
        let c = sphere_to_ray * sphere_to_ray - 1.0;
        let discriminant: f32 = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return Intersections::new(Vec::new());
        }
        Intersections::new(vec![
            Intersection {
                objectId: self.id,
                time: (-b - discriminant.sqrt()) / (2.0 * a),
            },
            Intersection {
                objectId: self.id,
                time: (-b + discriminant.sqrt()) / (2.0 * a),
            },
        ])
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}
