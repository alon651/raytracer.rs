use std::vec;

use crate::traits::intersect::Intersect;
use crate::tuple::Tuple;
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
impl Intersect for Sphere {
    fn calculate_intersection(&self, other: &crate::ray::Ray) -> Vec<f32> {
        let sphere_to_ray = other.origin - Tuple::new_point(0.0, 0.0, 0.0);
        let a = other.direction * other.direction;
        let b = 2.0 * (other.direction * sphere_to_ray);
        let c = sphere_to_ray * sphere_to_ray - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return Vec::new();
        }
        return vec![
            (-b - discriminant.sqrt()) / (2.0 * a),
            (-b + discriminant.sqrt()) / (2.0 * a),
        ];
    }
}
